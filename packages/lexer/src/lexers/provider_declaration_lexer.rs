use super::attribute_lexer::AttributeBlockLexer;
use crate::{
    chars::LexerChar,
    tokens::{LexerSymbol, LexerToken},
    Lexer,
};
use regex::Regex;

pub struct ProviderDeclarationLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for ProviderDeclarationLexer {
    fn lex(&mut self) -> usize {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();
        self.buffer.clear();
        let mut processed_count = 0;

        let whitespace_regex = Regex::new(r"\s+").unwrap();

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            processed_count += 1;

            if char == LexerChar::ProviderAssignmentColon.to_string() {
                // When we reach the colon, lex the var idenitifier and push it to the tokens before the colon
                let buffered = self.buffer.join("");
                if buffered.len() > 0 {
                    self.tokens.push(LexerToken::Identifier(buffered));
                } else {
                    panic!("Expected variable declaration identifier before colon")
                }
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::ProviderAssignmentColon));
                self.buffer.clear();
                continue;
            }

            if char == LexerChar::BlockOpenCurly.to_string() {
                // If we reach a block open curly, store current buffer and continue to lex for attributes
                let type_value = self.buffer_to_type();
                if type_value.len() > 0 {
                    self.tokens.push(LexerToken::ProviderType(type_value));
                }

                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::BlockOpenCurly));
                let sub_chars = &bound_chars[(index + 1)..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let mut block_lexer = AttributeBlockLexer::new(sub_chars);

                processed_count += block_lexer.lex();

                self.tokens.append(&mut block_lexer.tokens);

                return processed_count;
            }

            // Terminate lexing of provider declaration if we encounter a newline
            if char == LexerChar::NewLine.to_string() {
                let type_value = self.buffer_to_type();
                if type_value.len() > 0 {
                    self.tokens.push(LexerToken::ProviderType(type_value));
                }

                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::Whitespace));

                return processed_count;
            }

            if whitespace_regex.is_match(&char) {
                continue;
            }

            self.buffer.push(char);
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let type_value = self.buffer_to_type();
        if type_value.len() > 0 {
            self.tokens.push(LexerToken::ProviderType(type_value));
        }

        processed_count
    }
}

impl ProviderDeclarationLexer {
    pub fn new(chars: Vec<String>) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    fn buffer_to_type(&self) -> String {
        let buffered = self.buffer.join("");

        buffered
    }
}
