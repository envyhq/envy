use envy_lexer::{Lexer, SourceFileLexer};
use std::{env, fs};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].clone();
        fs::read_to_string(file).unwrap()
    } else {
        "var my_cool_var: string\nvar my_other_var: int".to_string()
    };

    let mut lexer = SourceFileLexer::new(input.as_str());

    let (processed_count, _) = lexer.lex();

    log::info!(
        "tokens: {:#?} - processed_count: {}",
        lexer.tokens,
        processed_count
    );
}
