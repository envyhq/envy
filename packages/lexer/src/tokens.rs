use std::fmt::Display;
use strum::{Display, EnumIter};

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
    Float,
}

impl Display for LexerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerType::String => write!(f, "str"),
            LexerType::Int => write!(f, "int"),
            LexerType::Float => write!(f, "float"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumIter)]
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

#[derive(Debug, PartialEq, Clone, EnumIter)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum LexerKeyword {
    VarModifierKeyword(LexerVarModifierKeyword),
    DeclarationKeyword(LexerDeclarationKeyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexerSymbol {
    VarAssignmentColon,
    ProviderAssignmentColon,
    AttributeAssignmentEquals,
    BlockOpenCurly,
    BlockCloseCurly,
    Whitespace,
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum LexerToken {
    Identifier(String),
    Keyword(LexerKeyword),
    Symbol(LexerSymbol),
    Type(LexerType),
    Literal(LexerLiteral),
    ProviderType(String),
}
