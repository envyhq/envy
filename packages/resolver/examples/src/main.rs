use nv_parser::{Parser, SourceFileParser};
use nv_provider_env::EnvProvider;
use nv_resolver::{Resolver, TreeResolver};
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut input = "var my_cool_var: string\nvar my_other_var: int".to_string();

    if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].to_string();
        input = fs::read_to_string(file).unwrap();
    }

    let mut parser = SourceFileParser::new(input);

    let processed_count = parser.parse();

    log::info!("ast processed_count: {}", processed_count);

    let resolver = Resolver {
        providers: vec![Box::new(EnvProvider {})],
    };

    let resolved = resolver.resolve(parser.ast).await;

    println!("resolver resolved: {:#?}", resolved);

    Ok(())
}
