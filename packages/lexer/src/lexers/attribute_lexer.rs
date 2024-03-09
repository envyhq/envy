use crate::{
    chars::LexerChar,
    tokens::{LexerLiteral, LexerLiteralBuiltin, LexerSymbol, LexerToken},
    Lexer,
};
use regex::Regex;
use strum::IntoEnumIterator;

pub struct AttributeBlockLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for AttributeBlockLexer {
    fn lex(&mut self) -> usize {
        let mut chars = self.chars.iter();
        self.buffer.clear();
        let mut processed_count = 0;

        let whitespace_regex = Regex::new(r"\s+").unwrap();

        while let Some(char) = chars.next() {
            let char = char.to_owned();

            processed_count += 1;

            // Terminate lexing of a single attribute but look for more attributes
            if char == LexerChar::NewLine.to_string() {
                let literal_value = self.buffer_to_literal();
                if let Some(literal_value) = literal_value {
                    self.tokens.push(LexerToken::Literal(literal_value));
                }
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::Whitespace));
                self.buffer.clear();
                continue;
            }

            if whitespace_regex.is_match(&char) {
                continue;
            }

            if char == LexerChar::AttributeAssignmentEquals.to_string() {
                // When we reach the equals, lex the attribute idenitifier and push it to the tokens before the equals
                let buffered = self.buffer.join("");
                if buffered.len() > 0 {
                    self.tokens.push(LexerToken::Identifier(buffered));
                } else {
                    panic!("Expected attribute identifier before equals")
                }
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::AttributeAssignmentEquals));
                self.buffer.clear();
                continue;
            }

            if char == LexerChar::BlockCloseCurly.to_string() {
                // Terminate lexing if we reach a block close curly
                let literal_value = self.buffer_to_literal();
                if let Some(literal_value) = literal_value {
                    self.tokens.push(LexerToken::Literal(literal_value));
                }

                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::BlockCloseCurly));

                return processed_count;
            }

            self.buffer.push(char);
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let literal_value = self.buffer_to_literal();
        if let Some(literal_value) = literal_value {
            self.tokens.push(LexerToken::Literal(literal_value));
        }

        processed_count
    }
}

impl AttributeBlockLexer {
    pub fn new(chars: Vec<String>) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    fn buffer_to_literal(&self) -> Option<LexerLiteral> {
        let buffered = self.buffer.join("");

        let builtin =
            LexerLiteralBuiltin::iter().find(|type_value| type_value.to_string() == buffered);

        if builtin.is_some() {
            return Some(LexerLiteral::Builtin(builtin.unwrap()));
        }

        if buffered.starts_with("\"") && buffered.ends_with("\"") {
            return Some(LexerLiteral::String(
                buffered[1..buffered.len() - 1].to_string(),
            ));
        }

        let parsed_number = buffered.parse::<u64>();
        if parsed_number.is_ok() {
            return Some(LexerLiteral::Integer(parsed_number.unwrap()));
        }

        let parsed_number = buffered.parse::<f64>();
        if parsed_number.is_ok() {
            return Some(LexerLiteral::Float(parsed_number.unwrap()));
        }

        None
    }
}
