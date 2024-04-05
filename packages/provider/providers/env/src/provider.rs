use std::env;

use nv_provider_core::{Provider, ProviderValueError};

pub struct EnvProvider {}

#[async_trait::async_trait]
impl Provider for EnvProvider {
    fn name(&self) -> &'static str {
        "env"
    }

    fn initialize(&self) -> () {}
    fn destroy(&self) -> () {}

    async fn get_value(&self, key: &str) -> Result<String, ProviderValueError> {
        env::var(key).map_err(|_| ProviderValueError::default())
    }
}
