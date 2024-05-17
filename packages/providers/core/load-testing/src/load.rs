use crate::types::DataCollection;
use futures::future::join_all;
use nv_provider_core::{async_trait, Controller, Message, Server, ServerError};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
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

pub static Y_DURATION_UNIT: &str = "µs";
pub static X_DURATION_UNIT: &str = "s";

pub async fn generate() -> Result<DataCollection, ServerError> {
    // let time = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let count = vec![100, 243, 123, 222, 312, 100, 243, 123, 222, 312];
    // let duration = vec![10, 24, 52, 92, 31, 30, 44, 122, 2, 61];

    // TODO: load test the socket server

    let controller = Arc::new(TestController {});
    let path = "/tmp/nv-provider.sock";

    let server = Server::new(path, controller);

    // TODO: should probably start the binary externally for this test?
    // then we can test isolated processes and release bin
    tokio::spawn(async move {
        let _ = server.start().await;
    });
    // TODO: no sleepy
    sleep(Duration::from_millis(100)).await;

    let msg = b"who";

    let time = Arc::new(Mutex::new(vec![]));
    let count = Arc::new(Mutex::new(vec![]));
    let duration = Arc::new(Mutex::new(vec![]));

    let root_start = Instant::now();
    let clients = Arc::new(Mutex::new(vec![]));

    while root_start.elapsed() <= Duration::from_secs(10) {
        // for _ in 0..1000 {
        let start = Instant::now();

        let client = UnixStream::connect(path).await;

        if client.is_err() {
            // return Err(ServerError::SocketError);
            continue;
        }
        let client = client.unwrap();

        let mut clients = clients.lock().await;
        clients.push(Arc::new(Mutex::new(client)));

        let mut futures = vec![];
        for client in clients.iter() {
            let client = client.clone();
            let count = count.clone();
            let time = time.clone();
            let duration = duration.clone();
            println!("TICK");
            let future = async move {
                println!("POW POW");
                let mut client = client.lock().await;
                client.write_all(msg).await.unwrap();

                let mut buf = [0; 1024];
                println!("get to reading...");
                let n = client.read(&mut buf).await.unwrap();
                println!("read {}...", n);

                let response = String::from_utf8((buf[..n]).to_vec());

                if response.unwrap() != "wow\n" {
                    return Err(ServerError::SocketError);
                }

                let root_elapsed = root_start.elapsed().as_secs_f64().round() as i64;

                let elapsed = start.elapsed().as_micros();
                let elapsed = i64::try_from(elapsed).unwrap();

                println!("GOT HERE");

                time.lock().await.push(root_elapsed);
                println!("GOT HERE 1");
                duration.lock().await.push(elapsed);
                println!("GOT HERE 2");
                let count_num = count.clone().lock().await.len() as i64;
                println!("GOT HERE 3");
                count.clone().lock().await.push(count_num);
                println!("GOT HERE 4");

                println!("DONE");

                Ok(())
            };

            println!("push it!!!");
            futures.push(future);
        }

        println!("join all...");

        let results = join_all(futures).await;
        println!("results: {:?}", results);
    }

    println!("WOWOWO");

    Ok((time, count, duration))
}
