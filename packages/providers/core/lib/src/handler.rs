use crate::errors::ServerError;
use crate::messages::Message;
use crate::Controller;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

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

        let read_out = req_tx.clone();
        let read_stream = self.stream.clone();
        let read_thread: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            loop {
                let ready = read_stream.readable().await;

                match ready {
                    Ok(_) => {
                        log::debug!("Client readable...");
                        let request = Handler::read_message(read_stream.clone()).await;
                        if let Ok(request) = request {
                            log::debug!("Sending request to write thread... {:?}", request);
                            read_out.send(request).await.unwrap();
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read readiness; err = {:?}", e);
                        break Err(ServerError::SocketError);
                    }
                }
            }
        });

        let mut action_in = req_rx;
        let action_out = res_tx;
        let controller = self.controller.clone();
        let action_thread: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            loop {
                while let Some(action) = action_in.recv().await {
                    log::debug!("Processing action {:?} of {}...", action, action_in.len());

                    if action == "quit\n".as_bytes() {
                        log::debug!("Client requested to quit...");
                        // TODO: we should exit gracefully here
                        return Err(ServerError::SocketError);
                    }

                    let response = controller.action(&action).await.unwrap();

                    let _ = action_out.send(response).await;
                }
            }
        });

        let write_stream = self.stream.clone();
        let mut write_in = res_rx;
        let write_thread: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            loop {
                while let Some(message) = write_in.recv().await {
                    log::debug!("Processing message {:?} of {}...", message, write_in.len());

                    // Only lock for read (no mutex behaviour) as unix sockets are full duplex per client
                    // and nv doesnt require managing strict ordering of messages
                    let ready = write_stream.writable().await;

                    match ready {
                        Ok(_) => {
                            log::debug!("Client writable...");
                            let result =
                                Handler::write_message(write_stream.clone(), message).await;
                            log::debug!("Wrote response {:?}", result);
                        }
                        Err(e) => {
                            log::error!("Failed to read readiness on write; err = {:?}", e);
                            return Err(ServerError::SocketError);
                        }
                    }
                }
            }
        });

        tokio::select! {
            _ = read_thread => {
                log::debug!("Read thread exited");
                    Ok(())
            }
            _ = action_thread => {
                log::debug!("Action thread exited");
                    Ok(())
            }
            _ = write_thread => {
                log::debug!("Write thread exited");
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
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                log::error!("Failed to read from socket; err = {:?}", e);
                Err(ServerError::Read)
            }
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
