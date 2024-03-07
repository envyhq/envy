use nv_lexer::{
    tokens::LexerDeclarationKeyword, Lexer, LexerKeyword, LexerToken, LexerType,
    LexerVarModifierKeyword, SourceFileLexer,
};

use crate::{parser::ParserResult, parsers::var_declaration_parser::VarDeclarationParser, Parser};

#[derive(Debug, Clone)]
pub struct VarDeclarationNode {
    pub identifier: String,
    pub type_value: LexerType,
    pub modifier: Option<LexerVarModifierKeyword>,
}

#[derive(Debug, Clone)]
pub struct PartialVarDeclarationNode {
    pub identifier: Option<String>,
    pub type_value: Option<LexerType>,
    pub modifier: Option<LexerVarModifierKeyword>,
}

impl TryInto<VarDeclarationNode> for PartialVarDeclarationNode {
    type Error = ();

    fn try_into(self) -> Result<VarDeclarationNode, Self::Error> {
        if self.identifier.is_none() || self.type_value.is_none() {
            return Err(());
        }

        Ok(VarDeclarationNode {
            identifier: self.identifier.unwrap(),
            type_value: self.type_value.unwrap(),
            modifier: self.modifier,
        })
    }
}

#[derive(Debug, Clone)]
pub enum DeclarationNode {
    VarDeclaration(VarDeclarationNode),
    ModuleDeclaration,
    ProviderDeclaration,
}

#[derive(Debug, Clone)]
struct SourceFileNode {
    pub declarations: Vec<DeclarationNode>,
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxNode {
    SourceFile(SourceFileNode),
    Declaration(DeclarationNode),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<AbstractSyntaxNode>,
}

#[derive(Debug)]
pub struct SourceFileParser {
    pub ast: AbstractSyntaxTree,
    pub tokens: Vec<LexerToken>,
}

impl Parser for SourceFileParser {
    fn parse(&mut self) -> usize {
        let bound_tokens = self.tokens.clone();
        let mut tokens = bound_tokens.iter().enumerate();

        let mut processed_count = 0;

        self.ast.root = Some(AbstractSyntaxNode::SourceFile(SourceFileNode {
            declarations: vec![],
        }));

        while let Some((index, token)) = tokens.next() {
            processed_count += 1;
            let sub_tokens = &bound_tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            log::debug!("SourceFileParser token: {:?} - {}", token, index);

            let result = match token {
                LexerToken::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerToken::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    log::debug!("SourceFileParser found var declaration");
                    let mut parser = VarDeclarationParser::new(sub_tokens.clone());
                    let count = parser.parse();

                    ParserResult {
                        processed_count: count,
                        ast_fragment: parser.ast_fragment,
                    }
                }
                _ => {
                    log::debug!("SourceFileParser found unknown token");

                    continue;
                }
            };

            log::debug!("11111 SourceFileParser result: {:#?} - {}", result, index);

            if let Some(ast_fragment) = result.clone().ast_fragment {
                if let AbstractSyntaxNode::Declaration(declaration) = ast_fragment {
                    if let AbstractSyntaxNode::SourceFile(source_file) =
                        &mut self.ast.root.as_mut().unwrap()
                    {
                        source_file.declarations.push(declaration);
                    }
                }
            }

            log::debug!("SourceFileParser result: {:#?} - {}", result, index);
        }

        processed_count
    }
}

impl SourceFileParser {
    pub fn new(input: String) -> Self {
        let mut lexer = SourceFileLexer::new(input);
        lexer.lex();

        Self {
            ast: AbstractSyntaxTree { root: None },
            tokens: lexer.tokens,
        }
    }
}
