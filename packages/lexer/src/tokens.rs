use std::fmt::Display;
use strum::EnumIter;

#[derive(Debug, PartialEq, Clone)]
pub enum LexerType {
    String,
    Int,
}

impl Display for LexerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerType::String => write!(f, "str"),
            LexerType::Int => write!(f, "int"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumIter)]
pub enum LexerKeyword {
    Pub,
    Var,
    Module,
    Provider,
}

impl Display for LexerKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerKeyword::Pub => write!(f, "pub"),
            LexerKeyword::Var => write!(f, "var"),
            LexerKeyword::Module => write!(f, "module"),
            LexerKeyword::Provider => write!(f, "provider"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerSymbol {
    VarAssignmentColon,
    AttributeAssignmentEquals,
    BlockOpenCurly,
    BlockCloseCurly,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerToken {
    Identifier(String),
    Keyword(LexerKeyword),
    Symbol(LexerSymbol),
    Type(LexerType),
}
