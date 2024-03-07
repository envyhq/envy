use std::convert::Infallible;

use log::debug;
use nv_lexer::{tokens::LexerSymbol, LexerKeyword, LexerToken, LexerVarModifierKeyword};

use crate::{
    parsers::source_file_parser::{DeclarationNode, PartialVarDeclarationNode, VarDeclarationNode},
    Parser,
};

use super::source_file_parser::AbstractSyntaxNode;

pub struct VarDeclarationParser {
    pub ast_fragment: Option<AbstractSyntaxNode>,
    pub tokens: Vec<LexerToken>,

    buffer: Vec<LexerToken>,
}

impl Parser for VarDeclarationParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration = PartialVarDeclarationNode {
            identifier: None,
            type_value: None,
            modifier: None,
        };

        while let Some((index, token)) = tokens.next() {
            let token = token.to_owned();

            processed_count += 1;

            log::debug!(
                "VarDeclarationParser token: {:?} -  buffer: {:?} - index: {}",
                token,
                self.buffer,
                index
            );

            match token {
                LexerToken::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);
                }
                LexerToken::Type(type_value) => {
                    partial_declaration.type_value = Some(type_value);
                }
                LexerToken::Keyword(LexerKeyword::VarModifierKeyword(modifier)) => {
                    partial_declaration.modifier = Some(modifier);
                }
                _ => {
                    log::debug!("VarDeclarationParser skipping token: {:?}", token);
                    self.buffer.push(token);
                }
            };

            let dec: Result<
                VarDeclarationNode,
                <PartialVarDeclarationNode as TryInto<VarDeclarationNode>>::Error,
            > = partial_declaration.clone().try_into();

            if dec.is_ok() {
                self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                    DeclarationNode::VarDeclaration(dec.unwrap()),
                ));
                return processed_count;
            } else {
                self.ast_fragment = None
            }
        }

        log::debug!(
            "VarDeclarationParser result: - processed_count: {} - ast_fragment: {:?}",
            processed_count,
            self.ast_fragment
        );

        processed_count
    }
}

impl VarDeclarationParser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            ast_fragment: None,
            tokens,
            buffer: vec![],
        }
    }
}
