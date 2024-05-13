use nv_provider_core::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new("/tmp/env.provider.nv.sock");

    let result = server.start().await;

    println!("Result: {:?}", result);
}
