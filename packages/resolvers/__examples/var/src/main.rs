use futures::executor;
use nv_lexer::{Lexer, SourceFileLexer};
use nv_parser::SourceFileParser;
use nv_var_resolver::{TreeResolver, VarResolver};
use std::{env, fs};

async fn async_main() -> Result<(), ()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut input = "var my_cool_var: string\nvar my_other_var: int".to_string();

    if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].to_string();
        input = fs::read_to_string(file).unwrap();
    }

    let mut lexer = SourceFileLexer::new(&input);
    lexer.lex();

    let (processed_count, ast) = SourceFileParser::parse(&lexer.tokens);

    log::info!("ast processed_count: {}", processed_count);

    let resolver = VarResolver::default();

    let resolved = resolver.resolve(ast.as_ref()).await;

    log::info!("{:#?}", resolved);

    Ok(())
}

fn main() {
    let _ = executor::block_on(async_main());
}
