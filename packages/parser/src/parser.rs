use crate::abstract_syntax_tree::AbstractSyntaxNode;
use nv_lexer::tokens::LexerToken;
use std::sync::Weak;

pub trait Parser<O> {
    fn parse(tokens: &Vec<LexerToken>, parent: Weak<AbstractSyntaxNode>) -> (usize, O);
}

#[derive(Debug)]
pub struct ParserResult {
    pub processed_count: usize,
    pub ast_fragment: Option<AbstractSyntaxNode>,
}
