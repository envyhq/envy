use crate::{
    chars::LexerChar,
    tokens::{LexerSymbol, LexerToken, LexerType},
    Lexer,
};

pub struct VarDeclarationLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for VarDeclarationLexer {
    // Called whenever the lexer encounters a var keyword, we continue to lex in the context of variable declaration.
    // We search for a colon to indicate the start of the variable type assignment, taking the previous string as identifier and the following string as type.
    fn lex(&mut self) -> usize {
        let mut chars = self.chars.iter();
        self.buffer.clear();
        let mut chars_processed_count = 0;

        while let Some(char) = chars.next() {
            let char = char.to_owned();

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
        let mut found = None;

        if buffered == LexerType::String.to_string() {
            found = Some(LexerType::String);
        }

        if buffered == LexerType::Int.to_string() {
            found = Some(LexerType::Int);
        }

        found
    }
}
