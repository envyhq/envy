use std::{iter::Enumerate, slice::Iter};

use super::utils::{is_newline, is_whitespace, lookbehind_raw_token};
use crate::{
    buffers::identifiers::buffer_to_keyword,
    chars::LexerChar,
    lexer::LexerResult,
    lexers::var_declaration_lexer::VarDeclarationLexer,
    tokens::{
        LexerDeclarationKeyword, LexerKeyword, LexerSymbol, LexerToken, LexerTokenKind,
        TokenPosition,
    },
    Lexer,
};

pub struct ModuleDeclarationLexer<'a> {
    pub tokens: Vec<LexerToken>,

    chars: &'a [String],
    buffer: Vec<String>,

    start_position: TokenPosition,
}

impl<'a> ModuleDeclarationLexer<'a> {
    pub fn new(chars: &'a [String], start_position: TokenPosition) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
            start_position,
        }
    }

    fn buffer_to_keyword(&self, buffer: &Option<Vec<String>>) -> Option<LexerKeyword> {
        buffer_to_keyword(&buffer.clone().unwrap_or(self.buffer.clone()))
    }

    fn lex_newline(&mut self, current_position: &mut TokenPosition) {
        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::Newline),
            current_position.clone(),
            current_position.clone(),
        ));

        current_position.line += 1;
        current_position.column = 0;
    }

    fn lex_block_open(&mut self, current_position: &mut TokenPosition) {
        let (buffer, from, to) = lookbehind_raw_token(
            current_position,
            &self.buffer,
            Some(LexerChar::BlockOpenCurly),
        );
        self.tokens.push(LexerToken::new(
            LexerTokenKind::Identifier(buffer.join("")),
            from,
            to,
        ));

        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::BlockOpenCurly),
            current_position.clone(),
            current_position.clone(),
        ));

        self.buffer.clear();
    }

    fn lex_block_close(&mut self, current_position: &mut TokenPosition) {
        self.tokens.push(LexerToken::new(
            LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
            current_position.clone(),
            current_position.clone(),
        ))
    }

    fn lex_keyword(
        &mut self,
        current_position: &mut TokenPosition,
        processed_count: &mut usize,
        index: &usize,
        chars: &mut Enumerate<Iter<String>>,
    ) {
        let (buffer, from, to) = lookbehind_raw_token(current_position, &self.buffer, None);
        if let Some(token) = self.buffer_to_keyword(&Some(buffer)) {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Keyword(token.clone()),
                from,
                to,
            ));
            let sub_chars = &self.chars[(index + 1)..];

            let mut result: LexerResult = match token {
                LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Var) => {
                    let mut lexer = VarDeclarationLexer::new(sub_chars, current_position.clone());
                    let (count, final_position) = lexer.lex();

                    LexerResult {
                        processed_count: count,
                        tokens: lexer.tokens,
                        final_position,
                    }
                }
                _ => {
                    self.buffer.clear();
                    return;
                }
            };

            self.tokens.append(&mut result.tokens);
            self.buffer.clear();

            *processed_count += result.processed_count;
            if result.processed_count > 0 {
                chars.nth(result.processed_count - 1);
            }
            *current_position = result.final_position.clone();
        }
    }
}

impl<'a> Lexer for ModuleDeclarationLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let mut chars = self.chars.iter().enumerate();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        while let Some((index, char)) = chars.next() {
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
                Ok(LexerChar::BlockOpenCurly) => {
                    self.lex_block_open(&mut current_position);
                    continue;
                }
                Ok(LexerChar::BlockCloseCurly) => {
                    self.lex_block_close(&mut current_position);
                    return (processed_count, current_position);
                }
                _ => {
                    self.lex_keyword(
                        &mut current_position,
                        &mut processed_count,
                        &index,
                        &mut chars,
                    );
                }
            }
        }

        self.lex_keyword(&mut current_position, &mut processed_count, &0, &mut chars);
        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::module_declaration_lexer::ModuleDeclarationLexer, Lexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;

    #[test]
    fn lexes_module_tokens() {
        let input = str_to_graphemes("Cakes {}");

        let start_line = 32;
        let start_column = 25;
        let mut lexer =
            ModuleDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(start_line, start_column + 8));
        assert_eq!(lexer.tokens.len(), 3);
        insta::assert_yaml_snapshot!(lexer.tokens);
    }

    #[test]
    fn lexes_module_tokens_with_vars() {
        let input = str_to_graphemes(
            "Cakes {
var my_cake_ingredient: str

pub var my_cake_url: url
}",
        );

        let start_line = 32;
        let start_column = 25;
        let mut lexer =
            ModuleDeclarationLexer::new(&input, TokenPosition::new(start_line, start_column));

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        assert_eq!(position, TokenPosition::new(start_line + 4, 1));
        assert_eq!(lexer.tokens.len(), 16);
        insta::assert_yaml_snapshot!(lexer.tokens);
    }
}
