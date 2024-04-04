#[derive(Debug, Default)]
pub struct ProviderValueError {
    pub message: Option<String>,
}

pub trait Provider {
    fn initialize(&self) -> ();
    fn destroy(&self) -> ();
    fn name(&self) -> &'static str;

    fn get_value(&self, key: &str) -> Result<String, ProviderValueError>;
}
