use super::{
    attribute_lexer::{lookbehind_raw_token, AttributeBlockLexer},
    utils::{is_newline, is_whitespace},
};
use crate::{
    chars::LexerChar,
    tokens::{LexerSymbol, LexerToken, LexerTokenKind, TokenPosition},
    Lexer,
};

pub struct ProviderDeclarationLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl ProviderDeclarationLexer {
    pub fn new(chars: Vec<String>, start_position: TokenPosition) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
            start_position,
        }
    }

    fn buffer_to_type(&self, buffer: Option<Vec<String>>) -> String {
        buffer.unwrap_or(self.buffer.clone()).join("")
    }
}

impl Lexer for ProviderDeclarationLexer {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let chars_iter = self.chars.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        for (index, char) in chars_iter {
            let char = char.to_owned();

            processed_count += 1;
            current_position.column += 1;

            self.buffer.push(char.clone());

            // Terminate lexing of provider declaration if we encounter a newline
            if is_newline(&char) {
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
                    &self.buffer,
                    Some(LexerChar::BlockOpenCurly),
                );
                let type_value = self.buffer_to_type(Some(buffer));

                if !type_value.is_empty() {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::ProviderType(type_value),
                        from,
                        to,
                    ));
                }

                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::Newline),
                    current_position.clone(),
                    current_position.clone(),
                ));
                current_position.line += 1;

                return (processed_count, current_position);
            }

            if is_whitespace(&char) {
                continue;
            }

            if char == LexerChar::ProviderAssignmentColon.to_string() {
                // When we reach the colon, lex the var idenitifier and push it to the tokens before the colon
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
                    &self.buffer,
                    Some(LexerChar::ProviderAssignmentColon),
                );
                let buffered = buffer.join("");
                if !buffered.is_empty() {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Identifier(buffered),
                        from,
                        to,
                    ));
                } else {
                    panic!("Expected variable declaration identifier before colon")
                }
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::ProviderAssignmentColon),
                    current_position.clone(),
                    current_position.clone(),
                ));
                self.buffer.clear();
                continue;
            }

            if char == LexerChar::BlockOpenCurly.to_string() {
                // If we reach a block open curly, store current buffer and continue to lex for attributes
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
                    &self.buffer,
                    Some(LexerChar::BlockOpenCurly),
                );
                let type_value = self.buffer_to_type(Some(buffer));
                if !type_value.is_empty() {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::ProviderType(type_value),
                        from,
                        to,
                    ));
                }

                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly),
                    current_position.clone(),
                    current_position.clone(),
                ));

                let sub_chars = &self.chars[(index + 1)..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let mut block_lexer = AttributeBlockLexer::new(sub_chars, current_position.clone());
                let (block_count, _) = block_lexer.lex();

                processed_count += block_count;

                self.tokens.append(&mut block_lexer.tokens);

                return (processed_count, current_position);
            }
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let (buffer, start_position, _) =
            lookbehind_raw_token(&current_position, &self.buffer, None);
        let type_value = self.buffer_to_type(Some(buffer));

        if !type_value.is_empty() {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::ProviderType(type_value),
                start_position.clone(),
                current_position.clone(),
            ));
        }

        (processed_count, current_position)
    }
}
