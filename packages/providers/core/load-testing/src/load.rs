use crate::types::DataCollection;
use nv_provider_core::{
    async_trait, Controller, Message, MessageDeserializer, MessageStreamReader, ProviderError,
    ServerError,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
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
    log::warn!("Please ensure the OS ulimit is increased for load testing. `ulimit -n 1048576`");

    let path = "/tmp/env.provider.nv.sock";

    let msg = b"HOME";

    let req_per_client = 10;
    let clients = Arc::new(Mutex::new(vec![]));

    let time = Arc::new(Mutex::new(vec![]));
    let req_count = Arc::new(Mutex::new(vec![]));
    let client_count = Arc::new(Mutex::new(vec![]));
    let duration = Arc::new(Mutex::new(vec![]));

    let root_start = Instant::now();

    let max_runs = 1.0; // or std::f32::INFINITY
    let mut runs_count = 0.0;

    while runs_count < max_runs && root_start.elapsed() <= Duration::from_secs(10) {
        runs_count += 1.0;
        log::debug!("Tick {} {:?}", runs_count, root_start.elapsed());

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
        let writable_client = client.clone();
        let mut writable_client = writable_client.lock().await;

        log::debug!("Writing msg... {:?}", msg);
        writable_client.write_all(msg).await.unwrap();

        std::mem::drop(writable_client);

        log::debug!("Reading response...");
        let message = MessageStreamReader::read_message(client).await.unwrap();

        log::debug!("Message read: {:?}", message);
        log::debug!("Length read: {:?}", message.len());

        let deserialized = MessageDeserializer::deserialize(&message);

        match deserialized {
            Ok(res) => {
                log::debug!("Deserialized header {:?}", res.0);
                log::debug!("Deserialized payload {:?}", res.1);
                log::debug!(
                    "Deserialized payload encoded UTF8 {:?}",
                    String::from_utf8(res.1)
                );
                // TODO: assert utf8 success?
            }

            Err(err) => {
                log::error!(
                    "Error deserializing message response in load test {:?}",
                    err
                );
            }
        }

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
