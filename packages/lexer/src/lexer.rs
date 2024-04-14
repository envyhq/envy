use crate::tokens::{LexerToken, TokenPosition};

pub trait Lexer {
    fn lex(&mut self) -> (usize, TokenPosition);
}

pub struct LexerResult {
    pub processed_count: usize,
    pub tokens: Vec<LexerToken>,
    pub final_position: TokenPosition,
}
