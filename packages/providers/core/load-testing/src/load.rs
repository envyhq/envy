use crate::types::DataCollection;
use nv_provider_core::{async_trait, Controller, Message, Server, ServerError};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::Instant;
use tokio::{net::UnixStream, time::sleep};

#[derive(Debug)]
struct TestController {}

#[async_trait]
impl Controller for TestController {
    async fn action(&self, message: &Message) -> Result<Message, ServerError> {
        Ok(message.clone())
    }
}

pub async fn generate() -> Result<DataCollection, ServerError> {
    // let time = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let count = vec![100, 243, 123, 222, 312, 100, 243, 123, 222, 312];
    // let duration = vec![10, 24, 52, 92, 31, 30, 44, 122, 2, 61];

    // TODO: load test the socket server

    let controller = Arc::new(TestController {});
    let path = "/tmp/nv-provider.sock";

    let server = Server::new(path, controller);

    tokio::spawn(async move {
        let _ = server.start().await;
    });
    sleep(Duration::from_millis(100)).await;

    let mut client = UnixStream::connect(path).await.unwrap();
    let msg = b"who";

    let start = Instant::now();

    client.write_all(msg).await.unwrap();

    let mut buf = [0; 1024];
    let n = client.read(&mut buf).await.unwrap();

    let response = String::from_utf8((buf[..n]).to_vec());

    if response.unwrap() != "wow\n" {
        return Err(ServerError::SocketError);
    }

    let elapsed = start.elapsed().as_millis();
    let elapsed = i64::try_from(elapsed).unwrap();

    let time = vec![0];
    let count = vec![1];
    let duration = vec![elapsed];

    println!("Elapsed time: {}ms", elapsed);

    Ok((time, count, duration))
}
