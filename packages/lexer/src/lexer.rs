pub trait Lexer {
    fn lex(&mut self) -> usize;
}
