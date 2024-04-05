#[derive(Debug, Default)]
pub struct ProviderValueError {
    pub message: Option<String>,
}

#[async_trait::async_trait]
pub trait Provider: Sync {
    fn initialize(&self) -> ();
    fn destroy(&self) -> ();
    fn name(&self) -> &'static str;

    async fn get_value(&self, key: &str) -> Result<String, ProviderValueError>;
}
