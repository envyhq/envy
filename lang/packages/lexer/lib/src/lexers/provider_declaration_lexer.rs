use super::{
    attribute_block_lexer::AttributeBlockLexer,
    utils::{is_newline, is_whitespace, lookbehind_raw_token},
};
use crate::{
    chars::LexerChar,
    tokens::{LexerSymbol, LexerToken, LexerTokenKind, TokenPosition},
    Lexer,
};

pub struct ProviderDeclarationLexer<'a> {
    pub tokens: Vec<LexerToken>,

    chars: &'a [String],
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl<'a> ProviderDeclarationLexer<'a> {
    pub fn new(chars: &'a [String], start_position: TokenPosition) -> Self {
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

impl<'a> Lexer for ProviderDeclarationLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let chars = self.chars.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        for (index, char) in chars {
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

                let sub_chars = &self.chars[(index + 1)..];

                let mut block_lexer = AttributeBlockLexer::new(sub_chars, current_position.clone());
                let (block_count, _) = block_lexer.lex();

                processed_count += block_count;

                self.tokens.append(&mut block_lexer.tokens);

                return (processed_count, current_position);
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

#[cfg(test)]
mod tests {
    use crate::{
        lexers::provider_declaration_lexer::ProviderDeclarationLexer, Lexer, TokenPosition,
    };
    use envy_unit_testing::str_to_graphemes;

    #[test]
    fn lexes_provider_tokens() {
        // We dont have "provider" keyword here, its handled by the source file lexer
        let input = str_to_graphemes("Env: env");

        let start_line = 20;
        let start_column = 5;
        let mut lexer =
            ProviderDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(start_line, start_column + 8));
        assert_eq!(lexer.tokens.len(), 3);
        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
