use crate::errors::ServerError;
use crate::messages::Message;
use crate::Controller;
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
}

impl Handler {
    pub fn new(stream: Arc<UnixStream>, controller: Arc<dyn Controller>) -> Self {
        Handler { stream, controller }
    }

    pub async fn handle(&self) -> Result<(), ServerError> {
        log::debug!("New client connected...");
        // TODO: think about whether or not we need to authenticate/authorize
        // new clients in any way. It will probably go here.
        //
        let (req_tx, req_rx) = mpsc::channel(16);
        let (res_tx, res_rx) = mpsc::channel(16);
        let cancel_token = CancellationToken::new();

        let read_out = req_tx.clone();
        let read_stream = self.stream.clone();
        let read_cancel = cancel_token.clone();
        let read_thread: JoinHandle<Result<(), ServerError>> = tokio::spawn(async move {
            tokio::select! {
                _ = read_cancel.cancelled() => {
                    log::debug!("Read thread cancelled");
                    Ok(())
                }
                _ = async {
                    let result: Result<(), ServerError> = loop {
                        let ready = read_stream.readable().await;

                        match ready {
                            Ok(_) => {
                                log::debug!("Client readable...");
                                let request = Handler::read_message(read_stream.clone()).await;

                                match request {
                                    Ok(request) => {
                                        log::debug!("Sending request to write thread... {:?}", request);
                                        let result = read_out.send(request).await;

                                        if let Err(e) = result {
                                            log::error!("Failed to write response; err = {:?}", e);
                                            break Err(ServerError::SocketError);
                                        }
                                    }
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
                    };

                    result
                } => {
                    log::debug!("Read thread aborted");
                    Ok(())
                }
            }
        });

        let mut action_in = req_rx;
        let action_out = res_tx;
        let controller = self.controller.clone();
        let action_cancel = cancel_token.clone();
        let action_thread: JoinHandle<Result<(), ServerError>> = tokio::spawn(async move {
            tokio::select! {
                _ = action_cancel.cancelled() => {
                    log::debug!("Action thread cancelled");
                    Ok(())
                }
                _ = async {
                    let res: Result<(), ServerError> = loop {
                        let message = action_in.recv().await;
                        if message.is_none() {
                            continue;
                        }

                        let action = message.unwrap();
                        log::debug!("Processing action {:?} of {}...", action, action_in.len());

                        if action == "quit\n".as_bytes() {
                            log::debug!("Client requested to quit...");
                            break Err(ServerError::SocketError);
                        }

                        let response = controller.action(&action).await.unwrap();

                        let result = action_out.send(response).await;

                        if result.is_err() {
                            log::error!("Failed to action message; err = {:?}", result);
                            break Err(ServerError::SocketError);
                        }
                    };

                    res
                } => {
                    log::debug!("Action thread aborted");
                    Ok(())
                }
            }
        });

        let write_stream = self.stream.clone();
        let mut write_in = res_rx;
        let write_cancel = cancel_token.clone();
        let write_thread: JoinHandle<Result<(), ServerError>> = tokio::spawn(async move {
            tokio::select! {
                _ = write_cancel.cancelled() => {
                    log::debug!("Write thread cancelled");
                    Ok(())
                }
                _ = async {
                    let res: Result<(), ServerError> = loop {
                        let message = write_in.recv().await;

                        if message.is_none() {
                            continue;
                        }

                        let message = message.unwrap();

                        log::debug!("Processing message {:?} of {}...", message, write_in.len());

                        let ready = write_stream.writable().await;

                        let result: Result<(), ServerError> = match ready {
                            Ok(_) => {
                                log::debug!("Client writable...");
                                let result = Handler::write_message(write_stream.clone(), message).await;
                                log::debug!("Wrote response {:?}", result);

                                if let Err(e) = result {
                                    log::error!("Failed to write response; err = {:?}", e);
                                    Err(ServerError::SocketError)
                                } else {
                                    Ok(())
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to read readiness on write; err = {:?}", e);
                                Err(ServerError::SocketError)
                            }
                        };

                        if result.is_err() {
                            println!("Result: {:?}", result);
                            break Err(ServerError::SocketError);
                        }
                    };

                    res
                } => {
                    log::debug!("Write thread cancelled");
                    Ok(())
                }

            }
        });

        tokio::select! {
            _ = read_thread => {
                log::debug!("Read thread exited");
                cancel_token.cancel();
                    Ok(())
            }
            _ = action_thread => {
                log::debug!("Action thread exited");
                cancel_token.cancel();
                    Ok(())
            }
            _ = write_thread => {
                log::debug!("Write thread exited");
                cancel_token.cancel();
                    Ok(())
            }
        }
    }

    async fn read_message(stream: Arc<UnixStream>) -> Result<Message, ServerError> {
        let mut message = vec![0; 1024];
        match stream.try_read(&mut message) {
            Ok(bytes_read) => {
                message.truncate(bytes_read);
                log::debug!("Read message: {:?}", String::from_utf8(message.to_owned()));

                Ok("wow\n".as_bytes().into())
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Err(ServerError::WouldBlock),
            Err(e) => {
                log::error!("Failed to read from socket; err = {:?}", e);
                Err(ServerError::Read)
            }
        }
    }

    async fn write_message(stream: Arc<UnixStream>, message: Message) -> Result<(), ServerError> {
        log::debug!("Trying to write {:?}", message);
        match stream.try_write(&message) {
            Ok(n) => {
                log::debug!("Wrote {} bytes", n);
                Ok(())
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                log::error!("Failed to write to socket; err = {:?}", e);
                Err(ServerError::Write)
            }
            Err(e) => {
                log::error!("Failed to write to socket; err = {:?}", e);
                Err(ServerError::Write)
            }
        }
    }
}
