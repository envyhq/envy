use crate::errors::ServerError;
use crate::messages::Message;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::mpsc::{self};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct Handler {
    pub stream: Arc<RwLock<UnixStream>>,
}

impl Handler {
    pub fn new(stream: Arc<RwLock<UnixStream>>) -> Self {
        Handler { stream }
    }

    pub async fn handle(&mut self) -> Result<(), ServerError> {
        log::debug!("New client connected...");
        // TODO: think about whether or not we need to authenticate/authorize
        // new clients in any way. It will probably go here.
        let (req_tx, mut req_rx) = mpsc::channel(32);
        // let (res_tx, mut res_rx) = mpsc::channel(32);

        let read_tx = req_tx.clone();
        let read_stream = self.stream.clone();
        let read_thread: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            loop {
                let ready = read_stream.read().await.readable().await;

                log::debug!("Client ready readable resolved...");
                match ready {
                    Ok(_) => {
                        let request = Handler::read_message(read_stream.clone()).await;
                        if let Ok(request) = request {
                            read_tx.send(request).await.unwrap();
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read readiness; err = {:?}", e);
                        break Err(ServerError::SocketError);
                    }
                }
            }
        });

        let write_stream = self.stream.clone();
        let mut write_rx = req_rx;
        let write_thread: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            loop {
                while let Some(message) = write_rx.recv().await {
                    println!("GOT = {:?}", String::from_utf8(message.clone()));

                    let ready = write_stream.read().await.writable().await;

                    log::debug!("Client ready writable resolved...");
                    match ready {
                        Ok(_) => {
                            let result =
                                Handler::write_message(write_stream.clone(), message).await;
                            println!("Result = {:?}", result);
                        }
                        Err(e) => {
                            log::error!("Failed to read readiness; err = {:?}", e);
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
            _ = write_thread => {
                log::debug!("Write thread exited");
                    Ok(())
            }
        }
    }

    async fn read_message(stream: Arc<RwLock<UnixStream>>) -> Result<Message, ServerError> {
        let mut message = vec![0; 1024];
        match stream.read().await.try_read(&mut message) {
            Ok(bytes_read) => {
                message.truncate(bytes_read);
                log::debug!("Message: {:?}", String::from_utf8(message.to_owned()));

                log::debug!("enqueuing...");
                log::debug!("enqued!!");

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

    async fn write_message(
        stream: Arc<RwLock<UnixStream>>,
        message: Message,
    ) -> Result<(), ServerError> {
        log::debug!("Trying to write {:?}", message);
        match stream.read().await.try_write(&message) {
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
