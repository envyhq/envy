use crate::{
    chars::LexerChar,
    lexers::utils::{is_newline, is_whitespace},
    tokens::{
        LexerLiteral, LexerLiteralBuiltin, LexerSymbol, LexerToken, LexerTokenKind, TokenPosition,
    },
    Lexer,
};
use strum::IntoEnumIterator;

pub struct AttributeBlockLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl AttributeBlockLexer {
    pub fn new(chars: Vec<String>, start_position: TokenPosition) -> Self {
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

    let from = TokenPosition::new(current_position.line, to_col - (trimmed_buffer.len() - 1));
    let to = TokenPosition::new(current_position.line, to_col);

    (trimmed_buffer, from, to)
}

impl Lexer for AttributeBlockLexer {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let mut chars = self.chars.iter();
        self.buffer.clear();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        while let Some(char) = chars.next() {
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
                if buffered.len() > 0 {
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
                let literal_value = self.buffer_to_literal(None);
                if let Some(literal_value) = literal_value {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Literal(literal_value.clone()),
                        TokenPosition::new(
                            current_position.line,
                            current_position.column - literal_value.to_string().len(),
                        ),
                        current_position.clone(),
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
        let literal_value = self.buffer_to_literal(None);
        if let Some(literal_value) = literal_value {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Literal(literal_value.clone()),
                TokenPosition::new(
                    current_position.line,
                    current_position.column - literal_value.to_string().len(),
                ),
                current_position.clone(),
            ));
        }

        (processed_count, current_position)
    }
}
