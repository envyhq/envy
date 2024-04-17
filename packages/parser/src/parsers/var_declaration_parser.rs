use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode, Leaf},
    parsers::attribute_block_parser::AttributeBlockParser,
    vars::{PartialVarDeclarationNode, VarDeclarationNode},
    Parser,
};
use nv_lexer::{
    tokens::{LexerSymbol, LexerToken},
    LexerKeyword, LexerTokenKind,
};
use std::sync::Weak;

pub struct VarDeclarationParser;

impl Parser<Option<AbstractSyntaxNode>> for VarDeclarationParser {
    fn parse(
        tokens: &Vec<LexerToken>,
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Option<AbstractSyntaxNode>) {
        let mut buffer = vec![];
        let mut tokens_iter = tokens.iter().enumerate();

        let mut ast_fragment = None;

        let mut processed_count = 0;

        let mut partial_declaration = PartialVarDeclarationNode {
            parent: parent.clone(),
            identifier: None,
            type_value: None,
            modifier: None,
            attributes: vec![],
        };

        while let Some((index, token)) = tokens_iter.next() {
            let token = token.to_owned();

            processed_count += 1;

            let sub_tokens = &tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            match token.kind {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(Leaf::new(identifier, token.range));

                    continue;
                }
                LexerTokenKind::Type(type_value) => {
                    partial_declaration.type_value = Some(Leaf::new(type_value, token.range));

                    continue;
                }
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(modifier)) => {
                    partial_declaration.modifier = Some(Leaf::new(modifier, token.range));

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let (count, parsed_block) =
                        AttributeBlockParser::parse(&sub_tokens, parent.clone());

                    // -1 because we dont want to double count the block open curly
                    let count = count - 1;

                    partial_declaration.attributes = parsed_block
                        .iter()
                        .map(|attribute| attribute.clone().into())
                        .collect();

                    processed_count += count;
                    if count > 0 {
                        tokens_iter.nth(count - 1);
                    }

                    let declaration: Result<VarDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::VarDeclaration(declaration.unwrap()),
                        ));
                    }
                }
                LexerTokenKind::Symbol(LexerSymbol::Newline) => {
                    let declaration: Result<VarDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::VarDeclaration(declaration.unwrap()),
                        ));
                    }
                }
                _ => {
                    buffer.push(token);
                    continue;
                }
            };

            if ast_fragment.is_some() {
                return (processed_count, ast_fragment);
            }
        }

        let declaration: Result<VarDeclarationNode, _> = partial_declaration.clone().try_into();

        if declaration.is_ok() {
            ast_fragment = Some(AbstractSyntaxNode::Declaration(
                DeclarationNode::VarDeclaration(declaration.unwrap()),
            ));

            return (processed_count, ast_fragment);
        }

        (processed_count, ast_fragment)
    }
}
