use crate::errors::ServerError;
use crate::handler::Handler;
use crate::messages::Message;
use crate::ProviderError;
use async_trait::async_trait;
use std::fmt::Debug;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;
use tokio::net::UnixListener;

#[async_trait]
pub trait Controller: Sync + Send + Debug {
    async fn action(&self, message: &Message) -> Result<Message, ProviderError>;
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

        clean_up();

        let listener = UnixListener::bind(self.path.clone()).unwrap();
        // TODO: move out of lib
        ctrlc::set_handler(move || {
            clean_up();
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            let socket = listener.accept().await;
            log::debug!("New client connected {:?}", socket);

            match socket {
                Ok((stream, _addr)) => {
                    let controller = self.controller.clone();
                    let stream = Arc::new(stream);
                    let handler = Arc::new(Handler::new(stream, controller));
                    tokio::spawn(async move { handler.handle().await });
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
    use crate::protocol::Opcode;
    use crate::{messages::Message, Server};
    use crate::{MessageDeserializer, MessageStreamReader, ProviderError};
    use async_trait::async_trait;
    use insta::assert_snapshot;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use tokio::net::UnixStream;
    use tokio::sync::Mutex;
    use tokio::time::sleep;

    #[derive(Debug)]
    struct TestController {}

    #[async_trait]
    impl Controller for TestController {
        async fn action(&self, message: &Message) -> Result<Message, ProviderError> {
            log::debug!("TestController received message {:?}", message);
            let response = "nout".as_bytes().to_vec();
            Ok(response)
        }
    }

    #[tokio::test]
    async fn test_server() {
        env_logger::init();

        let controller = Arc::new(TestController {});
        let path = "/tmp/nv-provider.sock";

        let server = Server::new(path, controller);

        tokio::spawn(async move {
            let _ = server.start().await;
        });
        sleep(Duration::from_millis(100)).await;

        let client = Arc::new(Mutex::new(UnixStream::connect(path).await.unwrap()));
        let msg = b"who";

        let writable_client = client.clone();
        let mut writable_client = writable_client.lock().await;
        writable_client.write_all(msg).await.unwrap();
        std::mem::drop(writable_client);

        log::debug!("Reading response...");
        let message = MessageStreamReader::read_message(&client).await.unwrap();

        log::debug!("Message read: {:?}", message);
        log::debug!("Length read: {:?}", message.len());

        let deserialized = MessageDeserializer::deserialize(&message);

        match deserialized {
            Ok(res) => {
                log::debug!("Deserialized header {:?}", res.header);
                log::debug!("Deserialized payload {:?}", res.payload);

                let header = res.header;
                let payload = res.payload;
                assert_eq!(header.version, 0x1);
                assert_eq!(header.opcode, Opcode::GetValue);
                assert_snapshot!(header.checksum);
                assert_eq!(header.payload_length, 4);
                assert_eq!(String::from_utf8(payload), Ok("nout".to_owned()));

                // TODO: verify checksum
            }

            Err(err) => {
                log::error!(
                    "Error deserializing message response in server test {:?}",
                    err
                );
                panic!("Deserialize should not error");
            }
        }
    }
}
