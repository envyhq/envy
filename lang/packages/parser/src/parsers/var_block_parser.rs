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
        tokens: &[LexerToken],
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Vec<AbstractSyntaxNode>) {
        let mut tokens_iter = tokens.iter().enumerate();
        let mut ast_block = vec![];

        let mut processed_count = 0;

        while let Some((index, token)) = tokens_iter.next() {
            processed_count += 1;

            let result = match token.kind {
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerTokenKind::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    let (count, parsed_fragment) =
                        VarDeclarationParser::parse(&tokens[index..], parent.clone());

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

#[cfg(test)]
mod tests {
    use crate::parsers::var_block_parser::{Parser, VarBlockParser};
    use nv_lexer::{Lexer, ModuleDeclarationLexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;
    use std::sync::Weak;

    #[test]
    fn parses_var_block_nodes() {
        let input = str_to_graphemes(
            "module MyCoolModule {
    var my_cool_var: str
    pub var my_other_var: str
}",
        );

        let start_line = 13;
        let start_column = 2;
        let mut lexer =
            ModuleDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));
        lexer.lex();

        let parent = Weak::new();
        let (count, ast) = VarBlockParser::parse(&lexer.tokens, parent);

        assert_eq!(count, 16);

        insta::assert_yaml_snapshot!(ast);
    }
}
