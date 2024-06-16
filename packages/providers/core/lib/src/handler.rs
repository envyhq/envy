use crate::errors::{ServerError, ServerResult};
use crate::messages::Message;
use crate::protocol::MessageSerializer;
use crate::{Controller, ProviderError};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct Handler {
    pub stream: Arc<UnixStream>,
    pub controller: Arc<dyn Controller>,
    pub serializer: MessageSerializer,
}

impl Handler {
    pub fn new(stream: Arc<UnixStream>, controller: Arc<dyn Controller>) -> Self {
        Handler {
            stream,
            controller,
            serializer: MessageSerializer {},
        }
    }

    // In parallel, we read and write from the full duplex stream when its ready for such op.
    // "Actions" are also processed in their own green thread to allow for actions of varying
    // durations to be processed concurrently without blocking queue I/O.
    pub async fn handle(self: Arc<Self>) -> Result<ServerResult, ServerError> {
        // TODO: think about whether or not we need to authenticate/authorize
        // new clients in any way. It will probably go here.
        // Had a thought today that the root problem is how to share a secret with both
        // the client app and the nv unix system. I thought about setting at compile/build time for
        // both but I feel like its essentially the same as saving to FS just w/ obfuscation.
        // We need a dynamic way to auth a client with server, that the server is actually the
        // inteded app. Maybe look into code signing for inspo?
        // .......
        // Lots more thinking today. Another idea...
        // We leave most of the auth to the providers. e.g. locally setting AWS creds in env vars
        // and conforming to AWS idea of identity on their machines.
        // Another potential to maybe harden is to write some secret token at startup or build time
        // that is consumed by the server and client once. This means only the first claim of the
        // container will have access for the rest of the duration of the container?
        // Also, we could ignore all the above and have some other agent/server that is
        // completely separate and private. This would essentially be stepping into the role of the providers
        // to some degree. For example, hashicorpt vault is secret management server that auths and comms with clients.

        let (req_tx, req_rx) = mpsc::channel(16);
        let (res_tx, res_rx) = mpsc::channel(16);
        let cancel_token = CancellationToken::new();

        let read_out = req_tx.clone();
        let read_stream = self.stream.clone();
        let read_cancel = cancel_token.clone();
        let read_thread: JoinHandle<Result<ServerResult, ServerError>> = tokio::spawn(async move {
            tokio::select! {
                _ = read_cancel.cancelled() => {
                    Ok(ServerResult::ClientDisconnected)
                }
                result = async {
                    loop {
                        let ready = read_stream.readable().await;

                        match ready {
                            Ok(_) => {
                                let request = Handler::read_message(read_stream.clone()).await;

                                log::debug!("Read message; request = {:?}", request);

                                match request {
                                    Ok(request) if !request.is_empty() => {
                                        let result = read_out.send(request.clone()).await;

                                        if let Err(e) = result {
                                            log::error!("{:?} Failed to send out read response; err = {:?}", String::from_utf8(request), e);
                                            break Err(ServerError::SocketError);
                                        }
                                    }
                                    Ok(_) => {
                                        // Empty message means client shutdown
                                        break Ok(ServerResult::ClientDisconnected);
                                    },
                                     Err(ServerError::WouldBlock) => {
                                        continue;
                                    }
                                    Err(e) => {
                                        log::error!("Failed to read message; err = {:?}", e);
                                        break Err(ServerError::SocketError);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to read readiness; err = {:?}", e);
                                break Err(ServerError::SocketError);
                            }
                        }
                    }
                } => {
                    result
                }
            }
        });

        let mut action_in = req_rx;
        let action_out = res_tx.clone();
        let controller = self.controller.clone();
        let action_cancel = cancel_token.clone();
        let action_thread: JoinHandle<Result<ServerResult, ServerError>> = tokio::spawn(
            async move {
                tokio::select! {
                    _ = action_cancel.cancelled() => {
                        Ok(ServerResult::CancelledGracefully)
                    }
                    result = async {
                        let res: Result<ServerResult, ServerError> = loop {
                            let message = action_in.recv().await;
                            if message.is_none() {
                                continue;
                            }

                            let action = message.unwrap();

                            let response = controller.action(&action).await;

                            let result = action_out.send(response).await;

                            if result.is_err() {
                                log::error!("Failed to action message; action = {:?} err = {:?}", String::from_utf8(action), result);
                                break Err(ServerError::SocketError);
                            }
                        };

                        res
                    } => {
                        result
                    }
                }
            },
        );

        let write_stream = self.stream.clone();
        let mut write_in = res_rx;
        let action_out_recover = res_tx;
        let write_cancel = cancel_token.clone();
        let handler: Arc<Handler> = Arc::clone(&self);
        let write_thread: JoinHandle<Result<ServerResult, ServerError>> = tokio::spawn(
            async move {
                tokio::select! {
                    _ = write_cancel.cancelled() => {
                        Ok(ServerResult::CancelledGracefully)
                    }
                    result = async {
                        loop {
                            let message = write_in.recv().await;

                            if message.is_none() {
                                continue;
                            }

                            let message = message.unwrap();


                            let ready = write_stream.writable().await;

                            match ready {
                                Ok(_) => {
                                    let result = handler.write_message(write_stream.clone(), message.clone()).await;

                                        match result {
                                            Err(ServerError::WouldBlock) => {
                                                // TODO: test that this is required
                                                let _ = action_out_recover.send(message).await;
                                                continue;
                                            }
                                            Err(e) => {
                                                log::error!("Failed to write response; err = {:?}", e);
                                                break Err(e)
                                            }
                                            Ok(_) => { continue; }
                                        }
                                }
                                Err(e) => {
                                    log::error!("Failed to read readiness on write; err = {:?}", e);
                                    break Err(ServerError::SocketError)
                                }
                            }
                        }
                    } => {
                        result
                    }

                }
            },
        );

        tokio::select! {
                result = read_thread => {
                    log::debug!("Read thread cancelled; result = {:?}", result);
                    cancel_token.cancel();
                    result.unwrap_or(Err(ServerError::SocketError))
                }
                result = action_thread => {
                    log::debug!("Action thread cancelled; result = {:?}", result);
                    cancel_token.cancel();
                    result.unwrap_or(Err(ServerError::SocketError))
                }
                result = write_thread => {
                    log::debug!("Write thread cancelled; result = {:?}", result);
                    cancel_token.cancel();
                    result.unwrap_or(Err(ServerError::SocketError))
                }
        }
    }

    async fn read_message(stream: Arc<UnixStream>) -> Result<Message, ServerError> {
        let mut message = vec![0; 1024];
        match stream.try_read(&mut message) {
            Ok(bytes_read) => {
                message.truncate(bytes_read);

                Ok(message)
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Err(ServerError::WouldBlock),
            Err(e) => {
                log::error!("Failed to read from socket; err = {:?}", e);
                Err(ServerError::Read)
            }
        }
    }

    async fn write_message(
        &self,
        stream: Arc<UnixStream>,
        res: Result<Message, ProviderError>,
    ) -> Result<(), ServerError> {
        let message = res.unwrap_or("nout".as_bytes().into());
        let serialized = self.serializer.serialize(message);

        match stream.try_write(&serialized) {
            Ok(_) => Ok(()),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Err(ServerError::Write),
            Err(e) => {
                log::error!("Failed to write to socket; err = {:?}", e);
                Err(ServerError::Write)
            }
        }
    }
}
