use crate::{
    chars::LexerChar,
    lexers::utils::{is_newline, is_whitespace, lookbehind_raw_token},
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

    fn lex_newline(&mut self, current_position: &mut TokenPosition) {
        if self.buffer.len() > 1 {
            let (buffer, from, to) = lookbehind_raw_token(
                current_position,
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
    }

    fn lex_attribute_assignment(&mut self, current_position: &mut TokenPosition) {
        let (buffer, from, to) = lookbehind_raw_token(
            current_position,
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
    }

    fn lex_block_close(&mut self, current_position: &mut TokenPosition) {
        self.lex_literal_value(current_position);

        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
            current_position.clone(),
            current_position.clone(),
        ));
    }

    fn lex_literal_value(&mut self, current_position: &mut TokenPosition) {
        let (buffer, from, to) = lookbehind_raw_token(
            current_position,
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
    }
}

impl<'a> Lexer for AttributeBlockLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let chars = self.chars.iter();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        for char in chars {
            processed_count += 1;
            current_position.column += 1;

            self.buffer.push(char.clone());

            if is_newline(char) {
                self.lex_newline(&mut current_position);
                continue;
            }

            if is_whitespace(char) {
                continue;
            }

            match char.parse() {
                Ok(LexerChar::AttributeAssignmentEquals) => {
                    self.lex_attribute_assignment(&mut current_position);
                    continue;
                }
                Ok(LexerChar::BlockCloseCurly) => {
                    self.lex_block_close(&mut current_position);
                    return (processed_count, current_position);
                }
                _ => {}
            }
        }

        self.lex_literal_value(&mut current_position);
        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::attribute_block_lexer::AttributeBlockLexer, Lexer, TokenPosition};
    use envy_unit_testing::str_to_graphemes;

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
attribute = 3.3
something = 4",
        );

        let mut lexer = AttributeBlockLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(3, 13));
        assert_eq!(lexer.tokens.len(), 15);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_builtin_attribute_tokens() {
        let input = str_to_graphemes(
            "attr = true
other = false
another = nowt",
        );

        let mut lexer = AttributeBlockLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(2, 14));
        assert_eq!(lexer.tokens.len(), 11);

        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
