use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    parsers::attribute_block_parser::AttributeBlockParser,
    providers::{PartialProviderDeclarationNode, ProviderDeclarationNode},
    Parser,
};
use nv_lexer::{tokens::LexerSymbol, LexerTokenKind};

pub struct ProviderDeclarationParser {
    pub ast_fragment: Option<AbstractSyntaxNode>,
    pub tokens: Vec<LexerTokenKind>,

    buffer: Vec<LexerTokenKind>,
}

impl Parser for ProviderDeclarationParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;

        let mut partial_declaration = PartialProviderDeclarationNode {
            identifier: None,
            type_value: None,
            attributes: vec![],
        };

        while let Some((index, token)) = tokens.next() {
            let token = token.to_owned();

            processed_count += 1;

            let sub_tokens = &bound_tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            match token {
                LexerTokenKind::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);

                    continue;
                }
                LexerTokenKind::ProviderType(type_value) => {
                    partial_declaration.type_value = Some(type_value);

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let mut parser = AttributeBlockParser::new(sub_tokens.clone());
                    // -1 because we dont want to double count the block open curly
                    let count = parser.parse() - 1;

                    partial_declaration.attributes = parser
                        .ast_block
                        .iter()
                        .map(|declaration| declaration.clone().into())
                        .collect();

                    processed_count += count;
                    if count > 0 {
                        tokens.nth(count - 1);
                    }

                    let declaration: Result<ProviderDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::ProviderDeclaration(declaration.unwrap().clone()),
                        ));
                        return processed_count;
                    }

                    continue;
                }
                LexerTokenKind::Symbol(LexerSymbol::Newline) => {
                    let declaration: Result<ProviderDeclarationNode, _> =
                        partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::ProviderDeclaration(declaration.unwrap().clone()),
                        ));

                        return processed_count;
                    }

                    continue;
                }
                _ => {
                    self.buffer.push(token);
                    continue;
                }
            };
        }

        let declaration: Result<ProviderDeclarationNode, _> =
            partial_declaration.clone().try_into();

        if declaration.is_ok() {
            self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                DeclarationNode::ProviderDeclaration(declaration.unwrap().clone()),
            ));

            return processed_count;
        }

        processed_count
    }
}

impl ProviderDeclarationParser {
    pub fn new(tokens: Vec<LexerTokenKind>) -> Self {
        Self {
            ast_fragment: None,
            tokens,
            buffer: vec![],
        }
    }
}
