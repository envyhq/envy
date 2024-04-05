use nv_provider_env::{EnvProvider, Provider};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let provider = EnvProvider {};

    let value = provider.get_value("HOME").await;

    log::debug!("provider value: {:#?}", value);

    Ok(())
}
