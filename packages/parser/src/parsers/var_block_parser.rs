use std::vec;

use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    parser::ParserResult,
    vars::{PartialVarDeclarationNode, VarDeclarationNode},
    Parser,
};
use nv_lexer::{
    tokens::LexerDeclarationKeyword, LexerKeyword, LexerToken, LexerVarModifierKeyword,
};

use super::var_declaration_parser::VarDeclarationParser;

pub struct VarBlockParser {
    pub ast_block: Vec<AbstractSyntaxNode>,
    pub tokens: Vec<LexerToken>,

    buffer: Vec<LexerToken>,
}

impl Parser for VarBlockParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration = PartialVarDeclarationNode {
            identifier: None,
            type_value: None,
            modifier: None,
            attributes: vec![],
        };

        while let Some((index, token)) = tokens.next() {
            processed_count += 1;
            let sub_tokens = &bound_tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            let result = match token {
                LexerToken::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerToken::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    let mut parser = VarDeclarationParser::new(sub_tokens.clone());
                    // -1 to avoid double counting the leading token (var or pub)
                    let count = parser.parse() - 1;

                    ParserResult {
                        processed_count: count,
                        ast_fragment: parser.ast_fragment,
                    }
                }
                _ => {
                    continue;
                }
            };

            let declaration: Result<VarDeclarationNode, _> = partial_declaration.clone().try_into();

            if declaration.is_ok() {
                self.ast_block.push(AbstractSyntaxNode::Declaration(
                    DeclarationNode::VarDeclaration(declaration.unwrap().clone()),
                ));
                partial_declaration = PartialVarDeclarationNode {
                    identifier: None,
                    type_value: None,
                    modifier: None,
                    attributes: vec![],
                };

                continue;
            }

            processed_count += result.processed_count;
            if result.processed_count > 0 {
                tokens.nth(result.processed_count - 1);
            }
        }

        processed_count
    }
}

impl VarBlockParser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            ast_block: vec![],
            tokens,
            buffer: vec![],
        }
    }
}
