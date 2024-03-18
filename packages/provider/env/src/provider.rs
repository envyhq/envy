use std::env;

use nv_provider_core::{Provider, ProviderValueError};

pub struct EnvProvider {}

impl Provider for EnvProvider {
    fn name(&self) -> String {
        "env".to_string()
    }

    fn initialize(&self) -> () {
        unimplemented!()
    }

    fn destroy(&self) -> () {
        unimplemented!()
    }

    fn get_value(&self, key: String) -> Result<String, ProviderValueError> {
        env::var(key).map_err(|_| ProviderValueError {})
    }
}
