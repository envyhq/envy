use crate::errors::ServerError;
use crate::handler::Handler;
use crate::messages::Message;
use std::fmt::Debug;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::RwLock;

trait Writer {
    fn write(&self, message: &Message);
}

trait Reader {
    fn read(&self) -> Message;
}

#[derive(Debug)]
pub struct Server {
    pub path: String,
    pub writer: Writer,
    pub reader: Writer,
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
                    tokio::spawn(async move {
                        Handler::new(Arc::new(RwLock::new(stream))).handle().await
                    });
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
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
