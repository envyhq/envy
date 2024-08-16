use nv_provider_core::{Controller, Message, ProviderError, Server};
use nv_provider_env::{EnvProvider, Provider};
use std::sync::Arc;

#[derive(Debug, Default)]
struct EnvController {
    provider: EnvProvider,
}

#[async_trait::async_trait]
impl Controller for EnvController {
    async fn action(&self, message: &Message) -> Result<Message, ProviderError> {
        let val = self
            .provider
            .get_value(String::from_utf8(message.clone()).unwrap().as_str())
            .await;

        log::debug!("EnvController val {:?}", val);

        val
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let controller = Arc::new(EnvController::default());

    let server = Server::new("/tmp/env.provider.nv.sock", controller);

    log::info!("Starting server...");

    let result = server.start().await;

    log::info!("Result: {:?}", result);
}
