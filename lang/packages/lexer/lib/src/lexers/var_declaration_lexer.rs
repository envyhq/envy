use super::{attribute_block_lexer::AttributeBlockLexer, utils::lookbehind_raw_token};
use crate::{
    chars::LexerChar,
    lexers::utils::{is_newline, is_whitespace},
    tokens::{LexerSymbol, LexerToken, LexerTokenKind, LexerType, TokenPosition},
    Lexer,
};
use strum::IntoEnumIterator;

pub struct VarDeclarationLexer<'a> {
    pub tokens: Vec<LexerToken>,

    chars: &'a [String],
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl<'a> VarDeclarationLexer<'a> {
    pub fn new(chars: &'a [String], start_position: TokenPosition) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
            start_position,
        }
    }

    fn buffer_to_type(&self, buffer: Vec<String>) -> Option<LexerType> {
        let buffered = buffer.join("");

        LexerType::iter().find(|type_value| type_value.to_string() == buffered)
    }

    fn lex_newline(&mut self, current_position: &mut TokenPosition) {
        self.lex_type_value(current_position, Some(LexerChar::NewLine));

        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::Newline),
            current_position.clone(),
            current_position.clone(),
        ));

        current_position.line += 1;
        current_position.column = 0;
    }

    fn lex_whitespace(&mut self, current_position: &mut TokenPosition) {
        self.lex_type_value(current_position, None);
        self.buffer.clear();
    }

    fn lex_block_close(&mut self, current_position: &mut TokenPosition) {
        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
            current_position.clone(),
            current_position.clone(),
        ));
    }

    fn lex_var_assignment(&mut self, current_position: &mut TokenPosition) {
        let (buffer, from, to) = lookbehind_raw_token(
            current_position,
            &self.buffer,
            Some(LexerChar::VarAssignmentColon),
        );

        let buffered = buffer.join("");
        if !buffered.is_empty() {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Identifier(buffered.clone()),
                from,
                to,
            ));
        } else {
            panic!("Expected variable declaration identifier before colon")
        }
        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::VarAssignmentColon),
            current_position.clone(),
            current_position.clone(),
        ));
        self.buffer.clear();
    }

    fn lex_block_open(
        &mut self,
        current_position: &mut TokenPosition,
        processed_count: &mut usize,
        index: &usize,
    ) {
        self.lex_type_value(current_position, Some(LexerChar::BlockOpenCurly));

        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly),
            current_position.clone(),
            current_position.clone(),
        ));

        let sub_chars = &self.chars[(index + 1)..];

        let mut block_lexer = AttributeBlockLexer::new(sub_chars, current_position.clone());

        let (block_count, block_position) = block_lexer.lex();

        *current_position = block_position;

        *processed_count += block_count;

        self.tokens.append(&mut block_lexer.tokens);
    }

    fn lex_type_value(
        &mut self,
        current_position: &mut TokenPosition,
        stop_char: Option<LexerChar>,
    ) {
        let (buffer, from, to) = lookbehind_raw_token(current_position, &self.buffer, stop_char);
        let type_value = self.buffer_to_type(buffer);
        if let Some(type_value) = type_value {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Type(type_value.clone()),
                from,
                to,
            ));
        }
    }
}

impl<'a> Lexer for VarDeclarationLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let chars = self.chars.iter().enumerate();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        for (index, char) in chars {
            let char = char.to_owned();

            processed_count += 1;
            current_position.column += 1;

            self.buffer.push(char.clone());

            if is_newline(&char) {
                self.lex_newline(&mut current_position);
                return (processed_count, current_position);
            }

            if is_whitespace(&char) {
                self.lex_whitespace(&mut current_position);
                continue;
            }

            match char.parse() {
                Ok(LexerChar::BlockCloseCurly) => {
                    self.lex_block_close(&mut current_position);
                    return (processed_count, current_position);
                }
                Ok(LexerChar::VarAssignmentColon) => {
                    self.lex_var_assignment(&mut current_position);
                    continue;
                }
                Ok(LexerChar::BlockOpenCurly) => {
                    self.lex_block_open(&mut current_position, &mut processed_count, &index);
                    return (processed_count, current_position);
                }
                _ => {
                    continue;
                }
            }
        }

        self.lex_type_value(&mut current_position, None);

        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::var_declaration_lexer::VarDeclarationLexer, Lexer, TokenPosition};
    use envy_unit_testing::str_to_graphemes;

    #[test]
    fn lexes_var_tokens() {
        // We dont have "var" keyword here, its handled by the source file lexer
        let input = str_to_graphemes("id: int");

        let start_line = 11;
        let start_column = 40;
        let mut lexer =
            VarDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(start_line, start_column + 7));
        assert_eq!(lexer.tokens.len(), 3);
        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
