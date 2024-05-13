use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::io::Interest;
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug)]
pub enum ServerError {
    Write,
    Read,
    NoMessages,
    SocketError,
}

#[derive(Debug)]
pub struct Server {
    pub path: String,
}

impl Server {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub async fn start(&self) -> Result<(), ServerError> {
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
                    tokio::spawn(async move { Handler::new(&stream).handle().await });
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        }
    }
}

type Message = Vec<u8>;
type Messages = Arc<Mutex<VecDeque<Message>>>;

#[derive(Debug)]
pub struct Handler<'a> {
    pub stream: &'a UnixStream,

    messages: Messages,
}

impl<'a> Handler<'a> {
    fn new(stream: &'a UnixStream) -> Self {
        Handler {
            stream,
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    async fn handle(&mut self) -> Result<(), ServerError> {
        log::debug!("New client connected...");

        loop {
            let ready = self
                .stream
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await;

            log::debug!("Client ready resolved...");
            match ready {
                Ok(ready) => {
                    log::debug!("Client is ready for...");

                    thread::scope(|scope| {
                        if ready.is_readable() {
                            scope.spawn(|| {
                                let _ = Handler::read_message(self.stream, self.messages.clone());
                            });
                        }

                        if ready.is_writable() {
                            scope.spawn(|| {
                                let _ = Handler::write_message(self.stream, self.messages.clone());
                            });
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to read readiness; err = {:?}", e);
                    break Err(ServerError::SocketError);
                }
            }
        }
    }

    fn read_message(stream: &UnixStream, messages: Messages) -> Result<(), ServerError> {
        let mut message = vec![0; 1024];
        match stream.try_read(&mut message) {
            Ok(bytes_read) => {
                message.truncate(bytes_read);
                log::debug!("Message: {:?}", String::from_utf8(message.to_owned()));

                log::debug!("enqueuing...");
                messages
                    .lock()
                    .unwrap()
                    .push_back("wow\n".as_bytes().into());
                log::debug!("enqued!!");

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

    fn write_message(stream: &UnixStream, messages: Messages) -> Result<(), ServerError> {
        log::debug!("Checking messages");
        let message = messages.lock().unwrap().pop_front();

        if message.is_none() {
            log::debug!("No messages");
            return Err(ServerError::NoMessages);
        }

        log::debug!("Trying to write {:?}", message);
        match stream.try_write(&message.unwrap()) {
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
    use crate::Server;

    #[tokio::test]
    async fn test_server() {
        let server = Server::new("/tmp/test2.sock");

        let result = server.start().await;

        println!("Result: {:?}", result);
    }
}
