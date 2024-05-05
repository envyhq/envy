use super::{attribute_block_lexer::AttributeBlockLexer, utils::lookbehind_raw_token};
use crate::{
    chars::LexerChar,
    lexers::utils::{is_newline, is_whitespace},
    tokens::{LexerSymbol, LexerToken, LexerTokenKind, LexerType, TokenPosition},
    Lexer,
};
use std::str::FromStr;
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

    fn buffer_to_type(&self) -> Option<LexerType> {
        let buffered = self.buffer.join("");

        LexerType::iter().find(|type_value| type_value.to_string() == buffered)
    }
}

impl<'a> Lexer for VarDeclarationLexer<'a> {
    // Called whenever the lexer encounters a var keyword, we continue to lex in the context of variable declaration, having already stored the var keyword token.
    // We search for a colon to indicate the start of the variable type assignment, taking everything before that as the identifier and everything after as the type.
    // We are given the whole source file from the var keyword onwards, so we lex until we reach a valid output.
    // The source file lexer must continue where we left of, so we return the number of characters we have processed.
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

            if is_newline(&char) {
                self.buffer.pop(); //  remove the newline
                let type_value = self.buffer_to_type();
                if let Some(type_value) = type_value {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Type(type_value.clone()),
                        TokenPosition::new(
                            current_position.line,
                            current_position.column - type_value.to_string().len(),
                        ),
                        TokenPosition::new(
                            current_position.line,
                            current_position.column - 1, // -1 because buffer.pop() to remove newline
                        ),
                    ));
                }

                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::Newline),
                    current_position.clone(),
                    current_position.clone(),
                ));

                current_position.line += 1;
                current_position.column = 0;

                return (processed_count, current_position);
            }

            if is_whitespace(&char) {
                self.buffer.pop(); //  remove the whitespace
                let type_value = self.buffer_to_type();
                if let Some(type_value) = type_value {
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Type(type_value.clone()),
                        TokenPosition::new(
                            current_position.line,
                            current_position.column - type_value.to_string().len(),
                        ),
                        TokenPosition::new(
                            current_position.line,
                            current_position.column - 1, // -1 because buffer.pop() to remove newline
                        ),
                    ));
                }

                self.buffer.clear();
                continue;
            }

            match LexerChar::from_str(&char) {
                Ok(LexerChar::BlockCloseCurly) => {
                    // Terminate lexing if we reach a block close curly, return
                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
                        current_position.clone(),
                        current_position.clone(),
                    ));

                    return (processed_count, current_position);
                }
                Ok(LexerChar::VarAssignmentColon) => {
                    // When we reach the colon, lex the var idenitifier and push it to the tokens before the colon
                    let (buffer, from, to) = lookbehind_raw_token(
                        &current_position,
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
                    continue;
                }
                Ok(LexerChar::BlockOpenCurly) => {
                    // If we reach a block open curly, check current buffer for type and continue to lex for attributes
                    let type_value = self.buffer_to_type();
                    if let Some(type_value) = type_value {
                        self.tokens.push(LexerToken::new(
                            LexerTokenKind::Type(type_value.clone()),
                            TokenPosition::new(
                                current_position.line,
                                current_position.column - type_value.to_string().len(),
                            ),
                            current_position.clone(),
                        ));
                    }

                    self.tokens.push(LexerToken::new(
                        LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly),
                        current_position.clone(),
                        current_position.clone(),
                    ));

                    let sub_chars = &self.chars[(index + 1)..];

                    let mut block_lexer =
                        AttributeBlockLexer::new(sub_chars, current_position.clone());

                    let (block_count, block_position) = block_lexer.lex();

                    current_position = block_position;

                    processed_count += block_count;

                    self.tokens.append(&mut block_lexer.tokens);

                    return (processed_count, current_position);
                }
                _ => {
                    continue;
                }
            }
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let type_value = self.buffer_to_type();
        if let Some(type_value) = type_value {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Type(type_value.clone()),
                TokenPosition::new(
                    current_position.line,
                    current_position.column + 1 - type_value.to_string().len(),
                ),
                current_position.clone(),
            ));
        }

        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::var_declaration_lexer::VarDeclarationLexer, Lexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;

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
