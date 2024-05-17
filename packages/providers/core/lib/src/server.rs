use crate::errors::ServerError;
use crate::handler::Handler;
use crate::messages::Message;
use async_trait::async_trait;
use std::fmt::Debug;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;
use tokio::net::UnixListener;

#[async_trait]
pub trait Controller: Sync + Send + Debug {
    async fn action(&self, message: &Message) -> Result<Message, ServerError>;
}

#[derive(Debug)]
pub struct Server {
    pub path: String,
    pub controller: Arc<dyn Controller>,
}

impl Server {
    pub fn new(path: &str, controller: Arc<dyn Controller>) -> Self {
        Self {
            path: path.to_owned(),
            controller,
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
                    let controller = self.controller.clone();
                    tokio::spawn(async move {
                        Handler::new(Arc::new(stream), controller).handle().await
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
    use super::Controller;
    use crate::{errors::ServerError, messages::Message, Server};
    use async_trait::async_trait;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::UnixStream;
    use tokio::time::sleep;

    #[derive(Debug)]
    struct TestController {}

    #[async_trait]
    impl Controller for TestController {
        async fn action(&self, message: &Message) -> Result<Message, ServerError> {
            Ok(message.clone())
        }
    }

    #[tokio::test]
    async fn test_server() {
        let controller = Arc::new(TestController {});
        let path = "/tmp/nv-provider.sock";

        let server = Server::new(path, controller);

        tokio::spawn(async move {
            let _ = server.start().await;
        });
        sleep(Duration::from_millis(100)).await;

        let mut client = UnixStream::connect(path).await.unwrap();
        let msg = b"who";
        client.write_all(msg).await.unwrap();

        let mut buf = [0; 1024];
        let n = client.read(&mut buf).await.unwrap();

        let response = String::from_utf8((buf[..n]).to_vec());

        assert_eq!(response, Ok("wow\n".to_owned()));
    }
}
