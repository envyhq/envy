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
use std::sync::{Arc, Weak};

pub struct VarDeclarationParser;

impl Parser<Option<AbstractSyntaxNode>> for VarDeclarationParser {
    fn parse(
        tokens: &[LexerToken],
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
            processed_count += 1;

            match &token.kind {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier =
                        Some(Leaf::new(identifier.clone(), token.range.clone()));

                    continue;
                }
                LexerTokenKind::Type(type_value) => {
                    partial_declaration.type_value =
                        Some(Leaf::new(type_value.clone(), token.range.clone()));

                    continue;
                }
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(modifier)) => {
                    partial_declaration.modifier =
                        Some(Leaf::new(modifier.clone(), token.range.clone()));

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let (count, parsed_block) =
                        AttributeBlockParser::parse(&tokens[index..], parent.clone());

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
                            DeclarationNode::VarDeclaration(Arc::new(declaration.unwrap())),
                        ));
                    }
                }
                LexerTokenKind::Symbol(LexerSymbol::Newline) => {
                    let declaration: Result<VarDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::VarDeclaration(Arc::new(declaration.unwrap())),
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
                DeclarationNode::VarDeclaration(Arc::new(declaration.unwrap())),
            ));

            return (processed_count, ast_fragment);
        }

        (processed_count, ast_fragment)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::var_declaration_parser::{Parser, VarDeclarationParser};
    use nv_lexer::{Lexer, TokenPosition, VarDeclarationLexer};
    use nv_unit_testing::str_to_graphemes;
    use std::sync::Weak;

    #[test]
    fn parses_var_nodes() {
        // We dont have "var" keyword here, its handled by the source file parser
        let input = str_to_graphemes("my_cool_var: str");

        let start_line = 10;
        let start_column = 5;
        let mut lexer =
            VarDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));
        lexer.lex();

        let parent = Weak::new();
        let (count, ast) = VarDeclarationParser::parse(&lexer.tokens, parent);

        assert_eq!(count, 3);

        insta::assert_yaml_snapshot!(ast);
    }
}
