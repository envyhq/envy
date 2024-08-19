use envy_parser::SourceFileParser;

pub fn parse(target_config: &str) {
    let (processed_count, ast) = SourceFileParser::from_file(target_config);

    println!(
        "ast:\n{:#?}\n\n---\n\ntotal steps:\n{}",
        ast, processed_count
    );
}
