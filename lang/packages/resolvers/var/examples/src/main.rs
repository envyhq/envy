use envy_parser::SourceFileParser;
use envy_var_resolver::{TreeResolver, VarResolver};
use futures::executor;
use std::env;

async fn async_main() -> Result<(), ()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].clone();

        file
    } else {
        "test.nv".to_string()
    };

    let (processed_count, ast) = SourceFileParser::from_file(&path);

    log::info!("ast processed_count: {}", processed_count);

    let resolver = VarResolver::default();

    let resolved = resolver.resolve_node(ast.as_ref()).await;

    log::info!("{:#?}", resolved);

    Ok(())
}

fn main() {
    let _ = executor::block_on(async_main());
}
