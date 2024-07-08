use futures::executor;
use nv_provider_aws_secrets_manager::{AwsSecretsManagerProvider, Provider};

async fn async_main() {
    env_logger::init();

    let provider = AwsSecretsManagerProvider {};

    let value = provider.get_value("TEST_SECRET").await;

    log::info!("provider value: {:#?}", value);
}

fn main() {
    executor::block_on(async_main());
}
