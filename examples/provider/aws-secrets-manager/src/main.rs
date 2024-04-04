use nv_provider_aws_secrets_manager::{AwsSecretsManagerProvider, Provider};

fn main() {
    env_logger::init();

    let provider = AwsSecretsManagerProvider {};

    let value = provider.get_value("TEST_SECRET");

    println!("provider value: {:#?}", value);
}
