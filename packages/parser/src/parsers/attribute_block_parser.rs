use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
    Parser,
};
use nv_lexer::{tokens::LexerSymbol, LexerTokenKind};

pub struct AttributeBlockParser {
    pub ast_block: Vec<AbstractSyntaxNode>,
    pub tokens: Vec<LexerTokenKind>,

    buffer: Vec<LexerTokenKind>,
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
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);
                }
                LexerTokenKind::Literal(value) => {
                    partial_declaration.value = Some(value);
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly) => {
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
    pub fn new(tokens: Vec<LexerTokenKind>) -> Self {
        Self {
            ast_block: vec![],
            tokens,
            buffer: vec![],
        }
    }
}
