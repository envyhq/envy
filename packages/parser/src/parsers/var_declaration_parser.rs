use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode},
    attributes::PartialAttributeDeclarationNode,
    parsers::attribute_parser::AttributeBlockParser,
    vars::{PartialVarDeclarationNode, VarDeclarationNode},
    Parser,
};
use nv_lexer::{tokens::LexerSymbol, LexerKeyword, LexerToken};

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
            attributes: vec![],
        };

        while let Some((index, token)) = tokens.next() {
            let token = token.to_owned();

            processed_count += 1;

            let sub_tokens = &bound_tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            match token {
                LexerToken::Identifier(identifier) => {
                    partial_declaration.identifier = Some(identifier);

                    continue;
                }
                LexerToken::Type(type_value) => {
                    partial_declaration.type_value = Some(type_value);

                    continue;
                }
                LexerToken::Keyword(LexerKeyword::VarModifierKeyword(modifier)) => {
                    partial_declaration.modifier = Some(modifier);

                    continue;
                }
                LexerToken::Symbol(LexerSymbol::BlockOpenCurly) => {
                    let mut parser = AttributeBlockParser::new(sub_tokens.clone());
                    // -1 because we dont want to double count the block open curly
                    let count = parser.parse() - 1;

                    partial_declaration.attributes = parser
                        .ast_block
                        .iter()
                        .filter_map(|f| match f {
                            AbstractSyntaxNode::Declaration(
                                DeclarationNode::AttributeDeclaration(attribute),
                            ) => Some(PartialAttributeDeclarationNode {
                                identifier: Some(attribute.clone().identifier),
                                value: Some(attribute.clone().value),
                            }),
                            _ => None,
                        })
                        .collect();

                    processed_count += count;
                    if count > 0 {
                        tokens.nth(count - 1);
                    }

                    let declaration: Result<
                        VarDeclarationNode,
                        <PartialVarDeclarationNode as TryInto<VarDeclarationNode>>::Error,
                    > = partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::VarDeclaration(declaration.unwrap().clone()),
                        ));
                        return processed_count;
                    }

                    continue;
                }
                LexerToken::Symbol(LexerSymbol::Whitespace) => {
                    let declaration: Result<
                        VarDeclarationNode,
                        <PartialVarDeclarationNode as TryInto<VarDeclarationNode>>::Error,
                    > = partial_declaration.clone().try_into();

                    if declaration.is_ok() {
                        self.ast_fragment = Some(AbstractSyntaxNode::Declaration(
                            DeclarationNode::VarDeclaration(declaration.unwrap().clone()),
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
