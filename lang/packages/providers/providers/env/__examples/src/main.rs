use futures::executor;
use nv_provider_env::{EnvProvider, Provider};

async fn async_main() -> Result<(), ()> {
    env_logger::init();

    let provider = EnvProvider {};

    let value = provider.get_value("HOME").await;

    log::info!("provider value: {:#?}", value);

    Ok(())
}

fn main() {
    let _ = executor::block_on(async_main());
}
