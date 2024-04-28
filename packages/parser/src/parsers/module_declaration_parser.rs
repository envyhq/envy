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
        tokens: &[LexerToken],
        parent: Weak<AbstractSyntaxNode>,
    ) -> (usize, Option<AbstractSyntaxNode>) {
        let mut buffer = vec![];
        let mut tokens_iter = tokens.iter().enumerate();

        buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration: PartialModuleDeclarationNode = PartialModuleDeclarationNode {
            identifier: None,
            declarations: vec![],
        };

        while let Some((index, token)) = tokens_iter.next() {
            processed_count += 1;

            match &token.kind {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier.clone());

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let (count, parsed_block) =
                        VarBlockParser::parse(&tokens[index..], parent.clone());

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

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly) => {
                    break;
                }
                _ => {
                    buffer.push(token);
                    continue;
                }
            };
        }

        let declaration: Result<ModuleDeclarationNode, _> = partial_declaration.clone().try_into();

        let node = declaration
            .ok()
            .map(|d| AbstractSyntaxNode::Declaration(DeclarationNode::ModuleDeclaration(d)));

        (processed_count, node)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::module_declaration_parser::{ModuleDeclarationParser, Parser};
    use nv_lexer::{Lexer, ModuleDeclarationLexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;
    use std::sync::Weak;

    #[test]
    fn parses_module_nodes() {
        // We dont have "module" keyword here, its handled by the source file parser
        let input = str_to_graphemes(
            "MyCoolModule {
    var my_var: url
}",
        );

        let start_line = 20;
        let start_column = 5;
        let mut lexer =
            ModuleDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));
        lexer.lex();

        let parent = Weak::new();
        let (count, ast) = ModuleDeclarationParser::parse(&lexer.tokens, parent);

        assert_eq!(count, 9);

        // TODO: assert line and column

        insta::assert_yaml_snapshot!(ast);
    }
}
