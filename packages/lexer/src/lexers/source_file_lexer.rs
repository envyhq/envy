use super::var_declaration_lexer::VarDeclarationLexer;
use crate::{
    buffers::identifiers::buffer_to_keyword,
    lexer::LexerResult,
    lexers::{
        module_declaration_lexer::ModuleDeclarationLexer,
        provider_declaration_lexer::ProviderDeclarationLexer,
        utils::{is_newline, is_whitespace},
    },
    tokens::{
        LexerDeclarationKeyword, LexerKeyword, LexerSymbol, LexerToken, LexerTokenKind,
        TokenPosition,
    },
    Lexer,
};
use unicode_segmentation::UnicodeSegmentation;

pub struct SourceFileLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl SourceFileLexer {
    pub fn new(input: &str) -> Self {
        let chars = input
            .graphemes(true)
            .map(|char| char.to_owned())
            .collect::<Vec<String>>();

        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }
    fn buffer_to_keyword(&self) -> Option<LexerKeyword> {
        buffer_to_keyword(&self.buffer)
    }
}

impl Lexer for SourceFileLexer {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let mut chars_iter = self.chars.iter().enumerate();

        if !self.tokens.is_empty() || !self.buffer.is_empty() {
            self.tokens = vec![];
            self.buffer = vec![];
        }

        let mut processed_count = 0;
        let mut current_position = TokenPosition::new(1, 0);

        while let Some((index, char)) = chars_iter.next() {
            let char = char.to_owned();

            processed_count += 1;
            current_position.column += 1;

            if is_newline(&char) {
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::Newline),
                    current_position.clone(),
                    current_position.clone(),
                ));
                current_position.line += 1;
                current_position.column = 0;

                continue;
            }

            if is_whitespace(&char) {
                continue;
            }

            self.buffer.push(char.clone());

            if let Some(token) = self.buffer_to_keyword() {
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Keyword(token.clone()),
                    TokenPosition {
                        line: current_position.line,
                        column: current_position
                            .column
                            .saturating_sub(token.to_string().len() - 1),
                    },
                    current_position.clone(),
                ));
                self.buffer.clear();
                let sub_chars = &self.chars[(index + 1)..];

                let result: LexerResult = match token {
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Var) => {
                        let mut lexer =
                            VarDeclarationLexer::new(sub_chars, current_position.clone());
                        let (count, final_position) = lexer.lex();

                        LexerResult {
                            processed_count: count,
                            tokens: lexer.tokens,
                            final_position,
                        }
                    }
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Module) => {
                        let mut lexer =
                            ModuleDeclarationLexer::new(sub_chars, current_position.clone());
                        let (count, final_position) = lexer.lex();

                        LexerResult {
                            processed_count: count,
                            tokens: lexer.tokens,
                            final_position,
                        }
                    }
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Provider) => {
                        let mut lexer =
                            ProviderDeclarationLexer::new(sub_chars, current_position.clone());
                        let (count, final_position) = lexer.lex();

                        LexerResult {
                            processed_count: count,
                            tokens: lexer.tokens,
                            final_position,
                        }
                    }
                    _ => {
                        continue;
                    }
                };

                self.tokens.append(&mut result.tokens.clone());

                processed_count += result.processed_count;
                if result.processed_count > 0 {
                    chars_iter.nth(result.processed_count - 1);
                }
                current_position = result.final_position;
                self.buffer.clear();

                continue;
            }
        }

        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::source_file_lexer::SourceFileLexer, Lexer, TokenPosition};

    #[test]
    fn lexes_source_file_tokens() {
        let input = "var my_test_var: int";

        let mut lexer = SourceFileLexer::new(input);

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(1, 20));
        assert_eq!(lexer.tokens.len(), 4);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_source_file_tokens_with_pub_var() {
        let input = "var my_test_var: int

pub var my_pub_test_var: str";

        let mut lexer = SourceFileLexer::new(input);

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(3, 28));
        assert_eq!(lexer.tokens.len(), 11);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_source_file_tokens_with_var_attributes() {
        let input = "var my_test_var: int

pub var my_pub_test_var: str
var my_attr_test_var: url {
    protocol = \"https\"
    host = \"test.com\"
}";

        let mut lexer = SourceFileLexer::new(input);

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(7, 1));
        assert_eq!(lexer.tokens.len(), 27);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_source_file_tokens_with_module() {
        let input = "var my_test_var: int

module TestModule {
    var my_test_var: int
}";

        let mut lexer = SourceFileLexer::new(input);

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(5, 1));
        assert_eq!(lexer.tokens.len(), 16);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_source_file_tokens_with_provider() {
        let input = "var my_test_var: int

provider Env: env

module TestModule {
    var my_test_var: int
}";

        let mut lexer = SourceFileLexer::new(input);

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(7, 1));
        assert_eq!(lexer.tokens.len(), 22);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
