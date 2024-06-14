use crate::types::DataCollection;
use nv_provider_core::{async_trait, Controller, Message, ProviderError, ServerError};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[derive(Debug)]
struct TestController {}

#[async_trait]
impl Controller for TestController {
    async fn action(&self, message: &Message) -> Result<Message, ProviderError> {
        Ok(message.clone())
    }
}

pub static Y_DURATION_UNIT: &str = "µs";
pub static X_DURATION_UNIT: &str = "ms";

pub async fn generate() -> Result<DataCollection, ServerError> {
    let path = "/tmp/env.provider.nv.sock";

    let msg = b"HOME";

    let req_per_client = 10;
    let clients = Arc::new(Mutex::new(vec![]));

    let time = Arc::new(Mutex::new(vec![]));
    let req_count = Arc::new(Mutex::new(vec![]));
    let client_count = Arc::new(Mutex::new(vec![]));
    let duration = Arc::new(Mutex::new(vec![]));

    let root_start = Instant::now();

    while root_start.elapsed() <= Duration::from_secs(10) {
        let start = Instant::now();

        let req_count_num = req_count.clone().lock().await.len() as i64;
        if req_count_num == 0 || req_count_num % req_per_client == 0 {
            let client = UnixStream::connect(path).await;
            if client.is_err() {
                log::debug!("BREAKING, stream err. {:?}", client);
                break;
            }

            let client = client.unwrap();
            clients.lock().await.push(Arc::new(Mutex::new(client)));
        }

        let client_count_num = clients.lock().await.len() as i64;
        client_count.lock().await.push(client_count_num);

        let clients = clients.lock().await;
        let client = clients.first().unwrap();
        let mut client = client.lock().await;

        client.write_all(msg).await.unwrap();

        let mut buf = [0; 1024];
        let n = client.read(&mut buf).await.unwrap();

        let response = String::from_utf8((buf[..n]).to_vec());
        log::debug!("Response received: {:?}", response);

        let root_elapsed = root_start.elapsed().as_millis() as i64;

        let elapsed = start.elapsed().as_micros();
        let elapsed = i64::try_from(elapsed).unwrap();

        time.lock().await.push(root_elapsed);
        duration.lock().await.push(elapsed);

        req_count.lock().await.push(req_count_num);
    }

    for client in clients.lock().await.iter() {
        let _ = client.lock().await.shutdown().await;
    }

    Ok((time, req_count, client_count, duration))
}
