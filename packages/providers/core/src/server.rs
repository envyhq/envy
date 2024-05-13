use async_trait::async_trait;
use std::fmt::Debug;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process::exit;
use tokio::io::Interest;
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug)]
pub struct Server {
    pub path: String,
    pub handler: Handler,
}

impl Server {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            handler: Handler::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Handler {}

#[async_trait]
pub trait SocketHandler {
    async fn handle_client(&self, stream: &UnixStream) -> Result<(), ServerError>;
    fn read_message(&self, message: &UnixStream) -> Result<(), ServerError>;
    fn write_message(&self, stream: &UnixStream) -> Result<(), ServerError>;
}

#[async_trait]
pub trait SocketServer {
    async fn start(&self) -> Result<(), ServerError>;
}

#[derive(Debug)]
pub enum ServerError {
    Write,
    Read,
}

#[async_trait]
impl SocketServer for Server {
    async fn start(&self) -> Result<(), ServerError> {
        let path = self.path.clone();
        let clean_up = move || {
            let result = remove_file(path.clone());

            if let Err(error) = result {
                if error.kind() == ErrorKind::NotFound {
                    return;
                }

                log::error!("Error removing file: {:?}", error);
            }
        };

        log::debug!("Cleaning up any existing socket file...");
        clean_up();

        log::debug!("Connecting socket...");
        let listener = UnixListener::bind(self.path.clone()).unwrap();

        ctrlc::set_handler(move || {
            log::debug!("Cleaning up...");
            clean_up();
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        log::debug!("Starting client accept loop...");

        loop {
            let socket = listener.accept().await;

            match socket {
                Ok((stream, _addr)) => {
                    let handler = self.handler.clone();
                    tokio::spawn(async move { handler.handle_client(&stream).await });
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        }
    }
}

#[async_trait]
impl SocketHandler for Handler {
    async fn handle_client(&self, stream: &UnixStream) -> Result<(), ServerError> {
        log::debug!("New client connected...");

        loop {
            let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await;

            log::debug!("Client ready resolved...");
            match ready {
                Ok(ready) => {
                    log::debug!("Client is ready for...");
                    if ready.is_readable() {
                        log::debug!("Reading!");
                        self.read_message(stream)?;
                    }

                    if ready.is_writable() {
                        log::debug!("Writing!");
                        self.write_message(stream)?;
                    }
                }
                Err(e) => {
                    log::error!("Failed to read readiness; err = {:?}", e);
                }
            }
        }
    }

    fn read_message(&self, stream: &UnixStream) -> Result<(), ServerError> {
        let mut message = vec![0; 1024];
        match stream.try_read(&mut message) {
            Ok(bytes_read) => {
                message.truncate(bytes_read);
                log::debug!("Message: {:?}", String::from_utf8(message.to_owned()));

                Ok(())
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

    fn write_message(&self, stream: &UnixStream) -> Result<(), ServerError> {
        match stream.try_write(b"hello world returned right back at you") {
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

#[cfg(test)]
mod tests {
    use crate::{Server, SocketServer};

    #[tokio::test]
    async fn test_server() {
        let server = Server::new("/tmp/test2.sock");

        let result = server.start().await;

        println!("Result: {:?}", result);
    }
}
