use super::attribute_lexer::AttributeLexer;
use crate::{
    chars::LexerChar,
    tokens::{LexerSymbol, LexerToken, LexerType},
    Lexer,
};
use regex::Regex;
use strum::IntoEnumIterator;

pub struct VarDeclarationLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for VarDeclarationLexer {
    // Called whenever the lexer encounters a var keyword, we continue to lex in the context of variable declaration, having already stored the var keyword token.
    // We search for a colon to indicate the start of the variable type assignment, taking everything before that as the identifier and everything after as the type.
    // We are given the whole source file from the var keyword onwards, so we lex until we reach a valid output.
    // The source file lexer must continue where we left of, so we return the number of characters we have processed.
    fn lex(&mut self) -> usize {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();
        self.buffer.clear();
        let mut chars_processed_count = 0;

        let whitespace_regex = Regex::new(r"\s+").unwrap();

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            println!(
                "VarDeclarationLexer char: {:?} WITH BUFFER {:?}",
                char, self.buffer
            );

            if char == LexerChar::VarAssignmentColon.to_string() {
                // When we reach the colon, lex the var idenitifier and push it to the tokens before the colon
                let buffered = self.buffer.join("");
                if buffered.len() > 0 {
                    self.tokens.push(LexerToken::Identifier(buffered));
                    chars_processed_count += self.buffer.len();
                } else {
                    panic!("Expected variable declaration identifier before colon")
                }
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::VarAssignmentColon));
                self.buffer.clear();
                chars_processed_count += self.buffer.len() + 1;
                chars.nth(0);
                continue;
            }

            if char == LexerChar::BlockOpenCurly.to_string() {
                // If we reach a block open curly, we will continue to lex for attributes
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::BlockOpenCurly));
                let sub_chars = &bound_chars[(index + 1)..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let mut attribute_lexer = AttributeLexer::new(sub_chars);
                chars_processed_count += attribute_lexer.lex();

                self.tokens.append(&mut attribute_lexer.tokens);

                return chars_processed_count;
            }

            // Terminate lexing of variable declaration if we encounter a newline
            if char == LexerChar::NewLine.to_string() {
                let type_value = self.buffer_to_type();
                if let Some(type_value) = type_value {
                    self.tokens.push(LexerToken::Type(type_value));
                    chars_processed_count += self.buffer.len() + 1;
                    chars.nth(0);
                }
                return chars_processed_count;
            }

            if whitespace_regex.is_match(&char) {
                continue;
            }

            self.buffer.push(char);
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let type_value = self.buffer_to_type();
        if let Some(type_value) = type_value {
            self.tokens.push(LexerToken::Type(type_value));
            chars_processed_count += 1;
        }

        chars_processed_count
    }
}

impl VarDeclarationLexer {
    pub fn new(chars: Vec<String>) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    fn buffer_to_type(&self) -> Option<LexerType> {
        let buffered = self.buffer.join("");

        LexerType::iter().find(|type_value| type_value.to_string() == buffered)
    }
}
