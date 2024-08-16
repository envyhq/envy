use serde::Serialize;
use std::fmt::Display;
use strum::{Display, EnumIter};

#[derive(Debug, PartialEq, Clone, EnumIter, Serialize)]
pub enum LexerLiteralBuiltin {
    True,
    False,
    Nowt,
}

impl Display for LexerLiteralBuiltin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerLiteralBuiltin::True => write!(f, "true"),
            LexerLiteralBuiltin::False => write!(f, "false"),
            LexerLiteralBuiltin::Nowt => write!(f, "nowt"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LexerLiteral {
    Builtin(LexerLiteralBuiltin),
    String(String),
    Float(f64),
    Integer(u64),
}

impl Display for LexerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerLiteral::Builtin(builtin) => write!(f, "{}", builtin),
            LexerLiteral::String(string) => write!(f, "\"{}\"", string),
            LexerLiteral::Float(float) => write!(f, "{}", float),
            LexerLiteral::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumIter, Serialize)]
pub enum LexerType {
    String,
    Int,
    Float,
    Url,
}

impl Display for LexerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerType::String => write!(f, "str"),
            LexerType::Int => write!(f, "int"),
            LexerType::Float => write!(f, "float"),
            LexerType::Url => write!(f, "url"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumIter, Serialize)]
pub enum LexerVarModifierKeyword {
    Pub,
}

impl Display for LexerVarModifierKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &LexerVarModifierKeyword::Pub => write!(f, "pub"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumIter, Serialize)]
pub enum LexerDeclarationKeyword {
    Var,
    Module,
    Provider,
}

impl Display for LexerDeclarationKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerDeclarationKeyword::Var => write!(f, "var"),
            LexerDeclarationKeyword::Module => write!(f, "module"),
            LexerDeclarationKeyword::Provider => write!(f, "provider"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LexerKeyword {
    VarModifierKeyword(LexerVarModifierKeyword),
    DeclarationKeyword(LexerDeclarationKeyword),
}

impl Display for LexerKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerKeyword::VarModifierKeyword(keyword) => write!(f, "{}", keyword),
            LexerKeyword::DeclarationKeyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LexerSymbol {
    VarAssignmentColon,
    ProviderAssignmentColon,
    AttributeAssignmentEquals,
    BlockOpenCurly,
    BlockCloseCurly,
    Newline,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Default, Serialize)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

impl TokenPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TokenRange {
    pub from: TokenPosition,
    pub to: TokenPosition,
}

impl TokenRange {
    pub fn new(from: TokenPosition, to: TokenPosition) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LexerToken {
    pub kind: LexerTokenKind,
    pub range: TokenRange,
}

impl LexerToken {
    pub fn new(kind: LexerTokenKind, from: TokenPosition, to: TokenPosition) -> Self {
        Self {
            kind,
            range: TokenRange::new(from, to),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display, Serialize)]
pub enum LexerTokenKind {
    Identifier(String),
    Keyword(LexerKeyword),
    Symbol(LexerSymbol),
    Type(LexerType),
    Literal(LexerLiteral),
    ProviderType(String),
}
