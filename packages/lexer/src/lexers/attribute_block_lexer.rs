use crate::{
    chars::LexerChar,
    lexers::utils::{is_newline, is_whitespace},
    tokens::{
        LexerLiteral, LexerLiteralBuiltin, LexerSymbol, LexerToken, LexerTokenKind, TokenPosition,
    },
    Lexer,
};
use strum::IntoEnumIterator;

pub struct AttributeBlockLexer<'a> {
    pub tokens: Vec<LexerToken>,

    chars: &'a [String],
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl<'a> AttributeBlockLexer<'a> {
    pub fn new(chars: &'a [String], start_position: TokenPosition) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
            start_position,
        }
    }

    fn buffer_to_literal(&self, buffer: Option<Vec<String>>) -> Option<LexerLiteral> {
        let buffered = buffer.unwrap_or(self.buffer.clone()).join("");

        let builtin =
            LexerLiteralBuiltin::iter().find(|type_value| type_value.to_string() == buffered);

        if let Some(builtin) = builtin {
            return Some(LexerLiteral::Builtin(builtin));
        }

        if buffered.starts_with('"') && buffered.ends_with('"') {
            return Some(LexerLiteral::String(
                buffered[1..buffered.len() - 1].to_string(),
            ));
        }

        let parsed_number = buffered.parse::<u64>();
        if let Ok(parsed_number) = parsed_number {
            return Some(LexerLiteral::Integer(parsed_number));
        }

        let parsed_number = buffered.parse::<f64>();
        if let Ok(parsed_number) = parsed_number {
            return Some(LexerLiteral::Float(parsed_number));
        }

        None
    }
}

pub fn lookbehind_raw_token(
    current_position: &TokenPosition,
    buffer: &[String],
    stop_char: Option<LexerChar>,
) -> (Vec<String>, TokenPosition, TokenPosition) {
    let last_identifier_char_index = buffer
        .iter()
        .rposition(|c| {
            !is_whitespace(c)
                && (stop_char.is_none() || *c != stop_char.as_ref().unwrap().to_string())
        })
        .unwrap_or(0);
    let trailing_trimmed = &buffer[..last_identifier_char_index + 1];
    let trimmed_buffer: Vec<String> = trailing_trimmed
        .iter()
        .map(|c| c.to_owned())
        .filter(|c| {
            !is_whitespace(c)
                && (stop_char.is_none() || *c != stop_char.as_ref().unwrap().to_string())
        })
        .collect();

    let to_col = current_position.column - (buffer.len() - (last_identifier_char_index + 1));

    let from = TokenPosition::new(
        current_position.line,
        to_col.saturating_sub(trimmed_buffer.len().saturating_sub(1)),
    );
    let to = TokenPosition::new(current_position.line, to_col);

    (trimmed_buffer, from, to)
}

impl<'a> Lexer for AttributeBlockLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let chars_iter = self.chars.iter();
        self.buffer.clear();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        for char in chars_iter {
            let char = char.to_owned();

            processed_count += 1;
            current_position.column += 1;

            self.buffer.push(char.clone());

            // Terminate lexing of a single attribute but look for more attributes
            if is_newline(&char) {
                if self.buffer.len() > 1 {
                    let (buffer, from, to) = lookbehind_raw_token(
                        &current_position,
                        &self.buffer,
                        Some(LexerChar::AttributeAssignmentEquals),
                    );

                    let literal_value = self.buffer_to_literal(Some(buffer));

                    if let Some(literal_value) = literal_value {
                        self.tokens.push(LexerToken::new(
                            LexerTokenKind::Literal(literal_value.clone()),
                            from,
                            to,
                        ));
                    }
                }
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::Newline),
                    current_position.clone(),
                    current_position.clone(),
                ));

                self.buffer.clear();
                current_position.line += 1;
                current_position.column = 0;
                continue;
            }

            if is_whitespace(&char) {
                continue;
            }

            if char == LexerChar::AttributeAssignmentEquals.to_string() {
                // When we reach the equals, lex the attribute idenitifier and push it to the tokens before the equals
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
                    &self.buffer,
                    Some(LexerChar::AttributeAssignmentEquals),
                );

                let buffered = buffer.join("");
                if !buffered.is_empty() {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Identifier(buffered.clone()),
                        from,
                        to,
                    ));
                } else {
                    panic!("Expected attribute identifier before equals")
                }
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::AttributeAssignmentEquals),
                    current_position.clone(),
                    current_position.clone(),
                ));
                self.buffer.clear();
                continue;
            }

            if char == LexerChar::BlockCloseCurly.to_string() {
                // Terminate lexing if we reach a block close curly
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
                    &self.buffer,
                    Some(LexerChar::BlockCloseCurly),
                );
                let literal_value = self.buffer_to_literal(Some(buffer));
                if let Some(literal_value) = literal_value {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Literal(literal_value.clone()),
                        from,
                        to,
                    ));
                }

                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
                    current_position.clone(),
                    current_position.clone(),
                ));

                return (processed_count, current_position);
            }
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let (buffer, from, to) = lookbehind_raw_token(&current_position, &self.buffer, None);
        let literal_value = self.buffer_to_literal(Some(buffer));
        println!("Literal value: {:?} - {:?}", literal_value, self.buffer);
        if let Some(literal_value) = literal_value {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Literal(literal_value.clone()),
                from,
                to,
            ));
        }

        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::attribute_block_lexer::AttributeBlockLexer, Lexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;

    #[test]
    fn lexes_attribute_tokens() {
        let input = str_to_graphemes(
            "unique = true
default = 2
attribute = 3
something = 4
}",
        );

        let mut lexer = AttributeBlockLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(4, 1));
        assert_eq!(lexer.tokens.len(), 17);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_inline_attribute_tokens() {
        let input = str_to_graphemes(
            "unique = true
default = 2
attribute = 3
something = 4 }",
        );

        let mut lexer = AttributeBlockLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(3, 15));
        assert_eq!(lexer.tokens.len(), 16);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_incomplete_attribute_tokens() {
        let input = str_to_graphemes(
            "unique = true
default = 2
attribute = 3
something = 4",
        );

        let mut lexer = AttributeBlockLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(3, 13));
        assert_eq!(lexer.tokens.len(), 15);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
