use crate::{
    tokens::{LexerKeyword, LexerToken},
    Lexer,
};
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

        if self.tokens.len() > 0 || self.buffer.len() > 0 {
            log::warn!("Lexer has already been used, clearing lexer state.");
            self.tokens = vec![];
            self.buffer = vec![];
        }

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            println!("char: {:?} WITH BUFFER {:?}", char, self.buffer);

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
}
