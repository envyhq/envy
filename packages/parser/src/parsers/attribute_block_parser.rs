use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode, Leaf},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
    Parser,
};
use nv_lexer::{
    tokens::{LexerSymbol, LexerToken},
    LexerTokenKind,
};
use std::sync::Weak;

pub struct AttributeBlockParser;

impl Parser<Vec<AbstractSyntaxNode>> for AttributeBlockParser {
    fn parse(
        tokens: &[LexerToken],
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Vec<AbstractSyntaxNode>) {
        let mut buffer = vec![];
        let mut ast_block: Vec<AbstractSyntaxNode> = vec![];
        let tokens = tokens.iter().enumerate();

        let mut processed_count = 0;

        let mut partial_declaration = PartialAttributeDeclarationNode {
            identifier: None,
            value: None,
        };

        for (_index, token) in tokens {
            let token = token.to_owned();

            processed_count += 1;

            match token.kind {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(Leaf::new(identifier, token.range));
                }
                LexerTokenKind::Literal(value) => {
                    partial_declaration.value = Some(Leaf::new(value, token.range));
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly) => {
                    break;
                }
                _ => {
                    buffer.push(token);
                }
            };

            let declaration: Result<AttributeDeclarationNode, _> =
                partial_declaration.clone().try_into();

            if let Ok(mut declaration) = declaration {
                declaration.parent = parent.clone();
                ast_block.push(AbstractSyntaxNode::Declaration(
                    DeclarationNode::AttributeDeclaration(declaration),
                ));

                partial_declaration = PartialAttributeDeclarationNode {
                    identifier: None,
                    value: None,
                };

                continue;
            }
        }

        (processed_count, ast_block)
    }
}
