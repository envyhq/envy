use super::{
    attribute_block_lexer::lookbehind_raw_token,
    utils::{is_newline, is_whitespace},
};
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
}

impl<'a> Lexer for ModuleDeclarationLexer<'a> {
    fn lex(&mut self) -> (usize, TokenPosition) {
        let mut chars = self.chars.iter().enumerate();

        self.buffer.clear();

        let mut processed_count = 0;
        let mut current_position = self.start_position.clone();

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            processed_count += 1;
            current_position.column += 1;

            self.buffer.push(char.clone());

            if is_newline(&char) {
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::Newline),
                    current_position.clone(),
                    current_position.clone(),
                ));

                current_position.line += 1;
                current_position.column = 0;
                continue;
            }

            if is_whitespace(&char) {
                continue;
            }

            if char == LexerChar::BlockOpenCurly.to_string() {
                // If we reach a block open curly, we will continue to lex for var declarations, taking everything before the block as the identifier
                let (buffer, from, to) = lookbehind_raw_token(
                    &current_position,
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
                continue;
            }

            // We do a very similar operation to SourceFileLexer here, lexing until we find a keyword and then passing the rest of the input to the appropriate lexer
            // The difference with modules is that they only contain var declarations as children
            // TODO: There is a way to re-use this util in both lexers
            let (buffer, from, to) = lookbehind_raw_token(&current_position, &self.buffer, None);
            if let Some(token) = self.buffer_to_keyword(&Some(buffer)) {
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Keyword(token.clone()),
                    from,
                    to,
                ));
                let sub_chars = &self.chars[(index + 1)..];

                let mut result: LexerResult = match token {
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Var) => {
                        let mut lexer =
                            VarDeclarationLexer::new(sub_chars, current_position.clone());
                        let (count, final_position) = lexer.lex();

                        LexerResult {
                            processed_count: count,
                            tokens: lexer.tokens,
                            final_position,
                        }
                    }
                    _ => {
                        self.buffer.clear();

                        continue;
                    }
                };

                self.tokens.append(&mut result.tokens);
                processed_count += result.processed_count;

                if result.processed_count > 0 {
                    chars.nth(result.processed_count - 1);
                }
                self.buffer.clear();
                current_position = result.final_position.clone();

                continue;
            }

            if char == LexerChar::BlockCloseCurly.to_string() {
                // Terminate lexing if we reach a block close curly
                self.tokens.push(LexerToken::new(
                    LexerTokenKind::Symbol(LexerSymbol::BlockCloseCurly),
                    current_position.clone(),
                    current_position.clone(),
                ));

                return (processed_count, current_position);
            }
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token, which can only be an identifier.
        if !self.buffer.is_empty() {
            self.tokens.push(LexerToken::new(
                LexerTokenKind::Identifier(self.buffer.join("")),
                current_position.clone(),
                current_position.clone(),
            ));
        }

        (processed_count, current_position)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexers::module_declaration_lexer::ModuleDeclarationLexer, Lexer, TokenPosition};
    use nv_unit_testing::str_to_graphemes;

    #[test]
    fn lexes_module_tokens() {
        let input = str_to_graphemes(
            "
module Cakes {}
",
        );

        let mut lexer = ModuleDeclarationLexer::new(&input, TokenPosition::default());

        let (count, position) = lexer.lex();

        assert_eq!(count, input.len());
        // WARN: explore why 0 line instead of 1? attr block lexer would treat this as line 1?
        assert_eq!(position, TokenPosition::new(0, 15));
        assert_eq!(lexer.tokens.len(), 4);
        insta::assert_debug_snapshot!(lexer.tokens);
    }
}
