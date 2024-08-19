use envy_lexer::{Lexer, SourceFileLexer};

pub fn lex(target_config: &str) {
    let mut lexer = SourceFileLexer::from_file(target_config);
    let (processed_count, _) = lexer.lex();

    println!(
        "tokens:\n{:#?}\n\n---\n\ntotal steps:\n{}",
        lexer.tokens, processed_count
    );
}
