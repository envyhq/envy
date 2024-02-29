use crate::tokens::LexerToken;

pub trait Lexer {
    fn lex(&mut self) -> usize;
}

pub struct LexerResult {
    pub processed_count: usize,
    pub tokens: Vec<LexerToken>,
}
