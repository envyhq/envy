use crate::{
    chars::LexerChar,
    tokens::{LexerKeyword, LexerSymbol, LexerToken},
    Lexer,
};
use regex::Regex;
use strum::IntoEnumIterator;
use unicode_segmentation::UnicodeSegmentation;

use super::var_declaration_lexer::VarDeclarationLexer;

pub struct SourceFileLexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer for SourceFileLexer {
    fn lex(&mut self) -> usize {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();

        let whitespace_regex = Regex::new(r"\s").unwrap();

        if self.tokens.len() > 0 || self.buffer.len() > 0 {
            log::warn!("Lexer has already been used, clearing lexer state.");
            self.tokens = vec![];
            self.buffer = vec![];
        }

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            if whitespace_regex.is_match(&char) {
                continue;
            }

            println!("char: {:?} WITH BUFFER {:?}", char, self.buffer);

            let symbol = self.char_to_symbol(&char);

            if symbol.is_some() {
                self.tokens.push(LexerToken::Symbol(symbol.unwrap()));
                self.buffer.clear();
                continue;
            }

            if let Some(token) = self.buffer_to_keyword() {
                self.tokens.push(LexerToken::Keyword(token.clone()));
                let sub_chars = &bound_chars[index..].to_vec();
                let sub_chars = sub_chars.to_vec();

                let mut lexer = match token {
                    LexerKeyword::Var => VarDeclarationLexer::new(sub_chars),
                    LexerKeyword::Module => VarDeclarationLexer::new(sub_chars),
                    LexerKeyword::Provider => VarDeclarationLexer::new(sub_chars),
                    _ => {
                        self.buffer.clear();
                        self.buffer.push(char);
                        continue;
                    }
                };

                let skip_count = lexer.lex();

                self.tokens.append(&mut lexer.tokens);

                if skip_count > 0 {
                    chars.nth(skip_count - 1);
                }

                self.buffer.clear();
                continue;
            }

            self.buffer.push(char);
        }

        self.chars.len()
    }
}

const LEXER_CHARS_LIST: [LexerChar; 2] = [LexerChar::BlockCloseCurly, LexerChar::BlockOpenCurly];
const LEXER_SYMBOLS_LIST: [LexerSymbol; 2] =
    [LexerSymbol::BlockCloseCurly, LexerSymbol::BlockOpenCurly];

impl SourceFileLexer {
    pub fn new(input: String) -> Self {
        let chars = input
            .graphemes(true)
            .map(|char| char.to_owned())
            .collect::<Vec<String>>();

        Self {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    fn buffer_to_keyword(&self) -> Option<LexerKeyword> {
        let buffered = self.buffer.join("");

        LexerKeyword::iter().find(|keyword| keyword.to_string() == buffered)
    }

    fn char_to_symbol(&self, char: &String) -> Option<LexerSymbol> {
        let result = LEXER_CHARS_LIST
            .iter()
            .map(|symbol| symbol.to_string())
            .enumerate()
            .find(|(_index, symbol)| symbol.to_string() == char.to_string());

        if result.is_none() {
            return None;
        }

        Some(LEXER_SYMBOLS_LIST[result.unwrap().0].clone())
    }
}
