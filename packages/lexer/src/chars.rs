use std::{fmt::Display, str::FromStr};

pub enum LexerChar {
    Space,
    VarAssignmentColon,
    ProviderAssignmentColon,
    NewLine,
    AttributeAssignmentEquals,
    BlockOpenCurly,
    BlockCloseCurly,
}

impl FromStr for LexerChar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(LexerChar::Space),
            ":" => Ok(LexerChar::VarAssignmentColon),
            "=" => Ok(LexerChar::AttributeAssignmentEquals),
            "{" => Ok(LexerChar::BlockOpenCurly),
            "}" => Ok(LexerChar::BlockCloseCurly),
            "\n" => Ok(LexerChar::NewLine),
            _ => Err(()),
        }
    }
}

impl Display for LexerChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerChar::Space => write!(f, " "),
            LexerChar::VarAssignmentColon => write!(f, ":"),
            LexerChar::ProviderAssignmentColon => write!(f, ":"),
            LexerChar::NewLine => writeln!(f),
            LexerChar::AttributeAssignmentEquals => write!(f, "="),
            LexerChar::BlockOpenCurly => write!(f, "{{"),
            LexerChar::BlockCloseCurly => write!(f, "}}"),
        }
    }
}
