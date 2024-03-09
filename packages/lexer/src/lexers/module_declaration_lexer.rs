use crate::{
    buffers::identifiers::buffer_to_keyword,
    chars::LexerChar,
    lexer::LexerResult,
    lexers::var_declaration_lexer::VarDeclarationLexer,
    tokens::{LexerDeclarationKeyword, LexerKeyword, LexerSymbol, LexerToken},
    Lexer,
};
use regex::Regex;

pub struct ModuleDeclarationLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for ModuleDeclarationLexer {
    fn lex(&mut self) -> usize {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();
        self.buffer.clear();
        let mut processed_count = 0;

        let whitespace_regex = Regex::new(r"\s+").unwrap();

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            processed_count += 1;

            if whitespace_regex.is_match(&char) {
                continue;
            }

            if char == LexerChar::BlockOpenCurly.to_string() {
                // If we reach a block open curly, we will continue to lex for var declarations, taking everything before the block as the identifier
                if self.buffer.len() > 0 {
                    self.tokens
                        .push(LexerToken::Identifier(self.buffer.join("")));
                }

                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::BlockOpenCurly));

                self.buffer.clear();
                continue;
            }

            // We do a very similar operation to SourceFileLexer here, lexing until we find a keyword and then passing the rest of the input to the appropriate lexer
            // The difference with modules is that they only contain var declarations as children
            // TODO: There is a way to re-use this util in both lexers
            if let Some(token) = self.buffer_to_keyword() {
                self.tokens.push(LexerToken::Keyword(token.clone()));
                let sub_chars = &bound_chars[index..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let mut result: LexerResult = match token {
                    LexerKeyword::DeclarationKeyword(LexerDeclarationKeyword::Var) => {
                        let mut lexer = VarDeclarationLexer::new(sub_chars);
                        let count = lexer.lex();

                        LexerResult {
                            processed_count: count,
                            tokens: lexer.tokens,
                        }
                    }
                    _ => {
                        self.buffer.clear();
                        self.buffer.push(char);

                        continue;
                    }
                };

                self.tokens.append(&mut result.tokens);
                processed_count += result.processed_count;

                if result.processed_count > 0 {
                    chars.nth(result.processed_count - 1);
                }
                self.buffer.clear();

                continue;
            }

            if char == LexerChar::BlockCloseCurly.to_string() {
                // Terminate lexing if we reach a block close curly
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::BlockCloseCurly));

                return processed_count;
            }

            self.buffer.push(char);
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token, which can only be an identifier.
        if self.buffer.len() > 0 {
            self.tokens
                .push(LexerToken::Identifier(self.buffer.join("")));
        }

        processed_count
    }
}

impl ModuleDeclarationLexer {
    pub fn new(chars: Vec<String>) -> Self {
        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    fn buffer_to_keyword(&self) -> Option<LexerKeyword> {
        buffer_to_keyword(&self.buffer)
    }
}
