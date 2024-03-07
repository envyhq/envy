use crate::parsers::source_file_parser::AbstractSyntaxNode;

pub trait Parser {
    fn parse(&mut self) -> usize;
}

#[derive(Debug, Clone)]
pub struct ParserResult {
    pub processed_count: usize,
    pub ast_fragment: Option<AbstractSyntaxNode>,
}
