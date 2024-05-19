use crate::types::DataCollection;
use nv_provider_core::{async_trait, Controller, Message, ServerError};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[derive(Debug)]
struct TestController {}

#[async_trait]
impl Controller for TestController {
    async fn action(&self, message: &Message) -> Result<Message, ServerError> {
        Ok(message.clone())
    }
}

pub static Y_DURATION_UNIT: &str = "µs";
pub static X_DURATION_UNIT: &str = "ms";

pub async fn generate() -> Result<DataCollection, ServerError> {
    let path = "/tmp/env.provider.nv.sock";

    let msg = b"who";

    let time = Arc::new(Mutex::new(vec![]));
    let count = Arc::new(Mutex::new(vec![]));
    let duration = Arc::new(Mutex::new(vec![]));

    let root_start = Instant::now();

    while root_start.elapsed() <= Duration::from_secs(10) {
        println!("WTF");
        let start = Instant::now();

        let client = UnixStream::connect(path).await;

        if client.is_err() {
            continue;
            // return Err(ServerError::SocketError);
        }
        let mut client = client.unwrap();

        println!("TICK");
        let count = count.clone();
        let time = time.clone();
        let duration = duration.clone();
        println!("TOCK");
        client
            .write_all(format!("{}{}", String::from_utf8(msg.to_vec()).unwrap(), 12345).as_bytes())
            .await
            .unwrap();

        println!("TOCK 0");

        let mut buf = [0; 1024];
        let n = client.read(&mut buf).await.unwrap();

        println!("TOCK 1");

        let _response = String::from_utf8((buf[..n]).to_vec());

        let root_elapsed = root_start.elapsed().as_millis() as i64;

        let elapsed = start.elapsed().as_micros();
        let elapsed = i64::try_from(elapsed).unwrap();

        println!("TOCK 1.5");

        time.lock().await.push(root_elapsed);
        println!("TOCK 2");
        duration.lock().await.push(elapsed);
        println!("TOCK 3");
        let count_num = count.clone().lock().await.len() as i64;
        println!("TOCK 4");
        count.clone().lock().await.push(count_num);

        println!("TOCK DONE");

        client.shutdown().await.unwrap();

        println!("JOINY DONE");
    }

    println!("DONE");

    Ok((time, count, duration))
}
