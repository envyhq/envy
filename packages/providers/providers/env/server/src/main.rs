use nv_provider_core::{Controller, Message, Server, ServerError};
use std::sync::Arc;

#[derive(Debug)]
struct TestController {}

#[async_trait::async_trait]
impl Controller for TestController {
    async fn action(&self, message: &Message) -> Result<Message, ServerError> {
        println!("CONTROLLER ACTION received message: {:?}", message);
        Ok(message.clone())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let controller = Arc::new(TestController {});

    let server = Server::new("/tmp/env.provider.nv.sock", controller);

    let result = server.start().await;

    println!("Result: {:?}", result);
}
