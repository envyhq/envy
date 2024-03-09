use nv_lexer::{Lexer, SourceFileLexer};
use std::{env, fs};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut input = "var my_cool_var: string\nvar my_other_var: int".to_string();

    if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].to_string();
        log::warn!("Reading from file! {}", file);
        input = fs::read_to_string(file).unwrap();
    }

    let mut lexer = SourceFileLexer::new(input);

    let processed_count = lexer.lex();

    log::info!(
        "tokens: {:#?} - processed_count: {}",
        lexer.tokens,
        processed_count
    );
}
