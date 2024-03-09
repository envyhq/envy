use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, AbstractSyntaxTree, SourceFileNode},
    parser::ParserResult,
    parsers::var_declaration_parser::VarDeclarationParser,
    Parser,
};
use nv_lexer::{
    tokens::LexerDeclarationKeyword, Lexer, LexerKeyword, LexerToken, LexerVarModifierKeyword,
    SourceFileLexer,
};

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

            if let Some(ast_fragment) = result.clone().ast_fragment {
                if let AbstractSyntaxNode::Declaration(declaration) = ast_fragment {
                    if let AbstractSyntaxNode::SourceFile(source_file) =
                        self.ast.root.as_mut().unwrap()
                    {
                        source_file.declarations.push(declaration);
                    }
                }
            }

            processed_count += result.processed_count;
            if result.processed_count > 0 {
                tokens.nth(result.processed_count - 1);
            }
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
