use nv_provider_env::{EnvProvider, Provider};

fn main() {
    env_logger::init();

    let provider = EnvProvider {};

    let value = provider.get_value("HOME");

    println!("provider value: {:#?}", value);
}
