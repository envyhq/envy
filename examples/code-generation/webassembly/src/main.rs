use nv_codegen_webassembly::generate_webassembly_source;
use nv_parser::{Parser, SourceFileParser};
use std::{env, fs};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut input = "var my_cool_var: string\nvar my_other_var: int".to_string();

    if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].to_string();
        input = fs::read_to_string(file).unwrap();
    }

    let mut parser = SourceFileParser::new(input);
    let processed_count = parser.parse();

    log::debug!(
        "ast: {:#?} - processed_count: {}",
        parser.ast,
        processed_count
    );

    let generated_source = generate_webassembly_source().unwrap();

    log::info!("generated source: {:#?}", generated_source);
}
