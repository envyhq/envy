use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
    Parser,
};
use nv_lexer::{tokens::LexerSymbol, LexerToken};

pub struct AttributeBlockParser {
    pub ast_block: Vec<AbstractSyntaxNode>,
    pub tokens: Vec<LexerToken>,

    buffer: Vec<LexerToken>,
}

impl Parser for AttributeBlockParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration = PartialAttributeDeclarationNode {
            identifier: None,
            value: None,
        };

        while let Some((_index, token)) = tokens.next() {
            let token = token.to_owned();

            processed_count += 1;

            match token {
                LexerToken::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);
                }
                LexerToken::Literal(value) => {
                    partial_declaration.value = Some(value);
                }
                LexerToken::Symbol(LexerSymbol::BlockCloseCurly) => {
                    break;
                }
                _ => {
                    self.buffer.push(token);
                }
            };

            let declaration: Result<AttributeDeclarationNode, _> =
                partial_declaration.clone().try_into();

            if declaration.is_ok() {
                self.ast_block.push(AbstractSyntaxNode::Declaration(
                    DeclarationNode::AttributeDeclaration(declaration.unwrap().clone()),
                ));
                partial_declaration = PartialAttributeDeclarationNode {
                    identifier: None,
                    value: None,
                };

                continue;
            }
        }

        processed_count
    }
}

impl AttributeBlockParser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            ast_block: vec![],
            tokens,
            buffer: vec![],
        }
    }
}
