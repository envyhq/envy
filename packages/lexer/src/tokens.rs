use std::fmt::Display;
use strum::EnumIter;

#[derive(Debug, PartialEq, Clone, EnumIter)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum LexerLiteral {
    Builtin(LexerLiteralBuiltin),
    String(String),
    Float(f64),
    Integer(u64),
}

#[derive(Debug, PartialEq, Clone, EnumIter)]
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
    ProviderAssignmentColon,
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
    Literal(LexerLiteral),
    ProviderType(String),
}
