use super::var_block_parser::VarBlockParser;
use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    modules::{ModuleDeclarationNode, PartialModuleDeclarationNode},
    Parser,
};
use nv_lexer::{
    tokens::{LexerSymbol, LexerToken},
    LexerTokenKind,
};
use std::sync::Weak;

pub struct ModuleDeclarationParser;

impl Parser<Option<AbstractSyntaxNode>> for ModuleDeclarationParser {
    fn parse(
        tokens: &Vec<LexerToken>,
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Option<AbstractSyntaxNode>) {
        let mut buffer = vec![];
        let mut ast_fragment = None;
        let mut tokens_iter = tokens.iter().enumerate();

        buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration: PartialModuleDeclarationNode = PartialModuleDeclarationNode {
            identifier: None,
            declarations: vec![],
        };

        while let Some((index, token)) = tokens_iter.next() {
            let token = token.to_owned();

            processed_count += 1;

            let sub_tokens = &tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            match token.kind {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let (count, parsed_block) = VarBlockParser::parse(&sub_tokens, parent.clone());

                    // -1 because we dont want to double count the block open curly
                    let count = count - 1;

                    partial_declaration.declarations = parsed_block
                        .iter()
                        .map(|declaration| declaration.clone().into())
                        .collect();

                    processed_count += count;
                    if count > 0 {
                        tokens_iter.nth(count - 1);
                    }

                    let declaration: Result<ModuleDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::ModuleDeclaration(declaration.unwrap()),
                        ));
                    }
                }
                LexerTokenKind::Symbol(LexerSymbol::Newline) => {
                    let declaration: Result<ModuleDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::ModuleDeclaration(declaration.unwrap()),
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

        let declaration: Result<ModuleDeclarationNode, _> = partial_declaration.clone().try_into();

        if declaration.is_ok() {
            ast_fragment = Some(AbstractSyntaxNode::Declaration(
                DeclarationNode::ModuleDeclaration(declaration.unwrap()),
            ));

            return (processed_count, ast_fragment);
        }

        (processed_count, ast_fragment)
    }
}
