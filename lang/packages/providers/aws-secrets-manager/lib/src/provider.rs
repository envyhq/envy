use envy_provider_core::{Provider, ProviderError};

#[derive(Debug)]
pub struct AwsSecretsManagerProvider {}

#[async_trait::async_trait]
impl Provider for AwsSecretsManagerProvider {
    fn name(&self) -> &'static str {
        "aws-secrets-manager"
    }

    fn initialize(&self) {}
    fn destroy(&self) {}

    async fn get_value(&self, key: &str) -> Result<Vec<u8>, ProviderError> {
        let config = aws_config::load_from_env().await;

        let value = aws_sdk_secretsmanager::Client::new(&config)
            .get_secret_value()
            .secret_id(key)
            .send()
            .await
            .map_err(|err| {
                log::error!("Aws env provider error: {:?}", err);

                if let Ok(_) = err.into_source() {
                    return ProviderError::Unknown("explodey".to_owned());
                }

                ProviderError::Unknown("explodey".to_owned())
            })
            .map(|response| {
                let res = response.secret_string.unwrap_or_default();
                let res = &res[..];
                let res = res.as_bytes();
                let res: Vec<_> = res.into();
                res
            });

        return value;
    }
}
