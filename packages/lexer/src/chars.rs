use std::fmt::Display;

pub enum LexerChar {
    Space,
    VarAssignmentColon,
    ProviderAssignmentColon,
    NewLine,
    AttributeAssignmentEquals,
    BlockOpenCurly,
    BlockCloseCurly,
}

impl Display for LexerChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerChar::Space => write!(f, " "),
            LexerChar::VarAssignmentColon => write!(f, ":"),
            LexerChar::ProviderAssignmentColon => write!(f, ":"),
            LexerChar::NewLine => write!(f, "\n"),
            LexerChar::AttributeAssignmentEquals => write!(f, "="),
            LexerChar::BlockOpenCurly => write!(f, "{{"),
            LexerChar::BlockCloseCurly => write!(f, "}}"),
        }
    }
}
