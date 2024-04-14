use crate::{abstract_syntax_tree::AbstractSyntaxNode, parser::ParserResult, Parser};
use nv_lexer::{
    tokens::LexerDeclarationKeyword, LexerKeyword, LexerTokenKind, LexerVarModifierKeyword,
};

use super::var_declaration_parser::VarDeclarationParser;

pub struct VarBlockParser {
    pub ast_block: Vec<AbstractSyntaxNode>,
    pub tokens: Vec<LexerTokenKind>,

    buffer: Vec<LexerTokenKind>,
}

impl Parser for VarBlockParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;

        while let Some((index, token)) = tokens.next() {
            processed_count += 1;
            let sub_tokens = &bound_tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            let result = match token {
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerTokenKind::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    let mut parser = VarDeclarationParser::new(sub_tokens.clone());
                    // -1 to avoid double counting the leading token (var or pub)
                    let count = parser.parse() - 1;

                    if let Some(ast_fragment) = parser.ast_fragment.clone() {
                        self.ast_block.push(ast_fragment);
                    }

                    ParserResult {
                        processed_count: count,
                        ast_fragment: parser.ast_fragment,
                    }
                }
                _ => {
                    continue;
                }
            };

            processed_count += result.processed_count;
            if result.processed_count > 0 {
                tokens.nth(result.processed_count - 1);
            }
        }

        processed_count
    }
}

impl VarBlockParser {
    pub fn new(tokens: Vec<LexerTokenKind>) -> Self {
        Self {
            ast_block: vec![],
            tokens,
            buffer: vec![],
        }
    }
}
