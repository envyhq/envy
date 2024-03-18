pub struct ProviderValueError {}

pub trait Provider {
    fn initialize(&self) -> ();
    fn destroy(&self) -> ();
    fn name(&self) -> String;

    fn get_value(&self, key: String) -> Result<String, ProviderValueError>;
}
