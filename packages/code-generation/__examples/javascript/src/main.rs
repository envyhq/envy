use nv_codegen_javascript::generate_javascript_source;
use nv_lexer::{Lexer, SourceFileLexer};
use nv_parser::SourceFileParser;
use std::{env, fs};

fn main() {
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

    log::debug!("ast: {:#?} - processed_count: {}", ast, processed_count);

    let generated_source = generate_javascript_source(nv_parser::AbstractSyntaxTree {
        root: Some(ast.as_ref().to_owned()),
    });

    log::info!("generated source: {:#?}", generated_source);
}
