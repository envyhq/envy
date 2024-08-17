use super::{
    module_declaration_parser::ModuleDeclarationParser,
    provider_declaration_parser::ProviderDeclarationParser,
};
use crate::{
    abstract_syntax_tree::{AbstractSyntaxNode, SourceFileNode},
    parser::ParserResult,
    parsers::var_declaration_parser::VarDeclarationParser,
    Parser,
};
use envy_lexer::{
    tokens::{LexerDeclarationKeyword, LexerToken},
    LexerKeyword, LexerTokenKind, LexerVarModifierKeyword,
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SourceFileParser;

impl SourceFileParser {
    pub fn parse(path: &str, tokens: &[LexerToken]) -> (usize, Arc<AbstractSyntaxNode>) {
        let root = Arc::new(AbstractSyntaxNode::SourceFile(Arc::new(SourceFileNode {
            path: path.to_owned(),
            declarations: Mutex::new(vec![]),
        })));
        let mut tokens_iter = tokens.iter().enumerate();

        let mut processed_count = 0;

        while let Some((index, token)) = tokens_iter.next() {
            processed_count += 1;
            let sub_tokens = &tokens[index..].to_vec();
            let sub_tokens = sub_tokens.to_vec();

            let result = match token.kind.clone() {
                LexerTokenKind::Keyword(LexerKeyword::VarModifierKeyword(
                    LexerVarModifierKeyword::Pub,
                ))
                | LexerTokenKind::Keyword(LexerKeyword::DeclarationKeyword(
                    LexerDeclarationKeyword::Var,
                )) => {
                    let (count, parsed_fragment) =
                        VarDeclarationParser::parse(&sub_tokens, Arc::downgrade(&root));

                    ParserResult {
                        processed_count: count - 1,
                        ast_fragment: parsed_fragment,
                    }
                }
                LexerTokenKind::Keyword(LexerKeyword::DeclarationKeyword(keyword)) => match keyword
                {
                    LexerDeclarationKeyword::Provider => {
                        let (count, parsed_fragment) =
                            ProviderDeclarationParser::parse(&sub_tokens, Arc::downgrade(&root));

                        ParserResult {
                            processed_count: count - 1,
                            ast_fragment: parsed_fragment,
                        }
                    }
                    LexerDeclarationKeyword::Module => {
                        let (count, parsed_fragment) =
                            ModuleDeclarationParser::parse(&sub_tokens, Arc::downgrade(&root));

                        ParserResult {
                            processed_count: count - 1,
                            ast_fragment: parsed_fragment,
                        }
                    }
                    _ => {
                        continue;
                    }
                },
                _ => {
                    continue;
                }
            };

            if let Some(AbstractSyntaxNode::Declaration(declaration)) = result.ast_fragment {
                if let AbstractSyntaxNode::SourceFile(source_node) = root.as_ref() {
                    source_node
                        .declarations
                        .lock()
                        .unwrap()
                        .push(Arc::new(declaration));
                }
            }

            processed_count += result.processed_count;
            if result.processed_count > 0 {
                tokens_iter.nth(result.processed_count - 1);
            }
        }

        (processed_count, root)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::source_file_parser::SourceFileParser;
    use envy_lexer::{Lexer, SourceFileLexer};

    #[test]
    fn parses_source_file_nodes() {
        let input = "var my_var: int

provider Env: env

module MyCoolModule {
    var my_cool_var: str
    pub var my_other_var: url
}";

        let mut lexer = SourceFileLexer::new(input);
        lexer.lex();

        let (count, ast) = SourceFileParser::parse("test.nv", &lexer.tokens);

        assert_eq!(count, 28);

        insta::assert_yaml_snapshot!(ast);
    }
}
