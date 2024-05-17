use nv_provider_core::ServerError;

mod charts;
mod load;
mod types;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();

    let (time, count, duration) = load::generate().await?;

    charts::generate(
        &time.lock().await,
        &count.lock().await,
        &duration.lock().await,
    );

    Ok(())
}
