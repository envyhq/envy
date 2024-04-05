use super::var_declaration_lexer::VarDeclarationLexer;
use crate::{
    buffers::identifiers::buffer_to_keyword,
    lexer::LexerResult,
    lexers::{
        module_declaration_lexer::ModuleDeclarationLexer,
        provider_declaration_lexer::ProviderDeclarationLexer,
    },
    tokens::{LexerDeclarationKeyword, LexerKeyword, LexerToken},
    Lexer,
};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub struct SourceFileLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for SourceFileLexer {
    fn lex(&mut self) -> usize {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();

        let whitespace_regex = Regex::new(r"\s+").unwrap();

        if self.tokens.len() > 0 || self.buffer.len() > 0 {
            self.tokens = vec![];
            self.buffer = vec![];
        }

        let mut processed_count = 0;

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            processed_count += 1;

            if whitespace_regex.is_match(&char) {
                continue;
            }

            if let Some(token) = self.buffer_to_keyword() {
                self.tokens.push(LexerToken::Keyword(token.clone()));
                let sub_chars = &bound_chars[index..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let result: LexerResult = match token {
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Var) => {
                        let mut lexer = VarDeclarationLexer::new(sub_chars);
                        let count = lexer.lex();

                        LexerResult {
                            processed_count: count - 1,
                            tokens: lexer.tokens,
                        }
                    }
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Module) => {
                        let mut lexer = ModuleDeclarationLexer::new(sub_chars);
                        let count = lexer.lex();

                        LexerResult {
                            processed_count: count - 1,
                            tokens: lexer.tokens,
                        }
                    }
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Provider) => {
                        let mut lexer = ProviderDeclarationLexer::new(sub_chars);
                        let count = lexer.lex();

                        LexerResult {
                            processed_count: count - 1,
                            tokens: lexer.tokens,
                        }
                    }
                    _ => {
                        self.buffer.clear();
                        self.buffer.push(char);

                        continue;
                    }
                };

                self.tokens.append(&mut result.tokens.clone());
                processed_count += result.processed_count;
                if result.processed_count > 0 {
                    chars.nth(result.processed_count - 1);
                }
                self.buffer.clear();

                continue;
            }

            self.buffer.push(char);
        }

        processed_count
    }
}

impl SourceFileLexer {
    pub fn new(input: String) -> Self {
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
