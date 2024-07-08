use std::fmt::Debug;

use crate::ProviderError;

#[async_trait::async_trait]
pub trait Provider: Sync + Debug {
    fn initialize(&self);
    fn destroy(&self);
    fn name(&self) -> &'static str;

    async fn get_value(&self, key: &str) -> Result<Vec<u8>, ProviderError>;
}
