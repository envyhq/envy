use crate::{abstract_syntax_tree::AbstractSyntaxNode, parser::ParserResult, Parser};
use nv_lexer::{
    tokens::{LexerDeclarationKeyword, LexerToken},
    LexerKeyword, LexerTokenKind, LexerVarModifierKeyword,
};
use std::sync::Weak;

use super::var_declaration_parser::VarDeclarationParser;

#[derive(Debug, Clone)]
pub struct VarBlockParser;

impl Parser<Vec<AbstractSyntaxNode>> for VarBlockParser {
    fn parse(
        tokens: &Vec<LexerToken>,
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Vec<AbstractSyntaxNode>) {
        let mut tokens_iter = tokens.iter().enumerate();
        let mut ast_block = vec![];

        let mut processed_count = 0;

        while let Some((index, token)) = tokens_iter.next() {
            processed_count += 1;
            let sub_tokens = &tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            let result = match token.kind {
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerTokenKind::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    let (count, parsed_fragment) =
                        VarDeclarationParser::parse(&sub_tokens, parent.clone());

                    // -1 to avoid double counting the leading token (var or pub)
                    let count = count - 1;

                    ParserResult {
                        processed_count: count,
                        ast_fragment: parsed_fragment,
                    }
                }
                _ => {
                    continue;
                }
            };

            processed_count += result.processed_count;
            if result.processed_count > 0 {
                tokens_iter.nth(result.processed_count - 1);
            }

            if result.ast_fragment.is_some() {
                ast_block.push(result.ast_fragment.unwrap());
            }
        }

        (processed_count, ast_block)
    }
}
