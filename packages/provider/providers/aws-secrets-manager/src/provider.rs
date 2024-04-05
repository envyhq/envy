use nv_provider_core::Provider;
use nv_provider_core::ProviderValueError;

pub struct AwsSecretsManagerProvider {}

#[async_trait::async_trait]
impl Provider for AwsSecretsManagerProvider {
    fn name(&self) -> &'static str {
        "aws-secrets-manager"
    }

    fn initialize(&self) -> () {}
    fn destroy(&self) -> () {}

    async fn get_value(&self, key: &str) -> Result<String, ProviderValueError> {
        println!("Getting value for key: {}", key);

        let config = aws_config::load_from_env().await;

        let value = aws_sdk_secretsmanager::Client::new(&config)
            .get_secret_value()
            .secret_id(key)
            .send()
            .await
            .map_err(|err| {
                if let Ok(source) = err.into_source() {
                    return ProviderValueError {
                        message: source.source().map(|error| error.to_string()),
                    };
                }

                ProviderValueError { message: None }
            })
            .map(|response| response.secret_string.unwrap_or_default());

        return value;
    }
}
