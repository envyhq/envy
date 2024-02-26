use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

enum LexerChar {
    Space,
    VarAssignmentColon,
    NewLine,
}

impl Display for LexerChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerChar::Space => write!(f, " "),
            LexerChar::VarAssignmentColon => write!(f, ":"),
            LexerChar::NewLine => write!(f, "\n"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerType {
    String,
    Int,
}

impl Display for LexerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerType::String => write!(f, "string"),
            LexerType::Int => write!(f, "int"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerKeyword {
    Var,
}

impl Display for LexerKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerKeyword::Var => write!(f, "var "),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerSymbol {
    VarAssignmentColon,
}

#[derive(Debug, PartialEq)]
pub enum LexerToken {
    Identifier(String),
    Keyword(LexerKeyword),
    Symbol(LexerSymbol),
    Type(LexerType),
}

pub struct Lexer {
    pub tokens: Vec<LexerToken>,

    chars: Vec<String>,
    buffer: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars = input
            .graphemes(true)
            .map(|char| char.to_owned())
            .collect::<Vec<String>>();

        Lexer {
            chars,
            tokens: vec![],
            buffer: vec![],
        }
    }

    pub fn lex(&mut self) {
        let bound_chars = self.chars.clone();
        let mut chars = bound_chars.iter().enumerate();

        if self.tokens.len() > 0 || self.buffer.len() > 0 {
            log::warn!("Lexer has already been used, clearing lexer state.");
            self.tokens = vec![];
            self.buffer = vec![];
        }

        while let Some((index, char)) = chars.next() {
            let char = char.to_owned();

            if char == LexerChar::VarAssignmentColon.to_string() {}

            if char == LexerChar::Space.to_string() {}

            if let Some(token) = self.buffer_to_keyword() {
                if token == LexerKeyword::Var {
                    self.tokens.push(LexerToken::Keyword(token));
                    let sub_chars = &bound_chars[index..];
                    let skip_count = self.lex_var_declaration(sub_chars);
                    if skip_count > 0 {
                        chars.nth(skip_count - 1);
                    }
                }

                self.buffer.clear();
                continue;
            }

            self.buffer.push(char);
        }
    }

    // Called whenever the lexer encounters a var keyword, we continue to lex in the context of variable declaration.
    // We search for a colon to indicate the start of the variable type assignment, taking the previous string as identifier and the following string as type.
    fn lex_var_declaration(&mut self, chars: &[String]) -> usize {
        let mut chars = chars.iter();
        self.buffer.clear();
        let mut skip_count = 0;

        while let Some(char) = chars.next() {
            let char = char.to_owned();

            if char == LexerChar::VarAssignmentColon.to_string() {
                // When we reach the colon, lex the var idenitifier and push it to the tokens before the colon
                let buffered = self.buffer.join("");
                if buffered.len() > 0 {
                    self.tokens.push(LexerToken::Identifier(buffered));
                    skip_count += self.buffer.len();
                } else {
                    panic!("Expected variable declaration identifier before colon")
                }
                self.tokens
                    .push(LexerToken::Symbol(LexerSymbol::VarAssignmentColon));
                self.buffer.clear();
                skip_count += self.buffer.len() + 1;
                chars.nth(0);
                continue;
            }

            // Terminate lexing of variable declaration if we encounter a newline
            if char == LexerChar::NewLine.to_string() {
                let type_value = self.buffer_to_type();
                if let Some(type_value) = type_value {
                    self.tokens.push(LexerToken::Type(type_value));
                    skip_count += self.buffer.len() + 1;
                    chars.nth(0);
                }
                return skip_count;
            }

            self.buffer.push(char);
        }

        // If we haven't returned already, we reached the end of the input. Check the buffer for any remaining token.
        let type_value = self.buffer_to_type();
        if let Some(type_value) = type_value {
            self.tokens.push(LexerToken::Type(type_value));
            skip_count += 1;
        }

        skip_count
    }

    fn buffer_to_keyword(&self) -> Option<LexerKeyword> {
        let buffered = self.buffer.join("");
        let mut found = None;

        if buffered == LexerKeyword::Var.to_string() {
            found = Some(LexerKeyword::Var);
        }

        found
    }

    fn buffer_to_type(&self) -> Option<LexerType> {
        let buffered = self.buffer.join("");
        let mut found = None;

        if buffered == LexerType::String.to_string() {
            found = Some(LexerType::String);
        }

        if buffered == LexerType::Int.to_string() {
            found = Some(LexerType::Int);
        }

        found
    }
}
