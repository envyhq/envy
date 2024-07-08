use crate::{
    attributes::AttributeDeclarationNode, modules::ModuleDeclarationNode,
    providers::ProviderDeclarationNode, vars::VarDeclarationNode,
};
use core::fmt;
use nv_lexer::{
    tokens::{LexerLiteral, TokenRange},
    LexerType, LexerVarModifierKeyword,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Leaf<V> {
    pub value: V,
    pub range: TokenRange,
}
impl<V> Leaf<V> {
    pub fn new(value: V, range: TokenRange) -> Self {
        Self { value, range }
    }
}
impl<V: fmt::Display> fmt::Display for Leaf<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub type Identifier = Leaf<String>;
pub type Literal = Leaf<LexerLiteral>;
pub type Type = Leaf<LexerType>;
pub type ProviderType = Leaf<String>;
pub type Modifier = Leaf<LexerVarModifierKeyword>;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DeclarationNode {
    VarDeclaration(Arc<VarDeclarationNode>),
    ModuleDeclaration(ModuleDeclarationNode),
    ProviderDeclaration(ProviderDeclarationNode),
    AttributeDeclaration(AttributeDeclarationNode),
}

impl From<AbstractSyntaxNode> for DeclarationNode {
    fn from(declaration: AbstractSyntaxNode) -> Self {
        if let AbstractSyntaxNode::Declaration(DeclarationNode::VarDeclaration(var)) = declaration {
            return var.as_ref().clone().into();
        }

        panic!("Invalid conversion");
    }
}

impl From<VarDeclarationNode> for DeclarationNode {
    fn from(declaration: VarDeclarationNode) -> Self {
        DeclarationNode::VarDeclaration(Arc::new(declaration))
    }
}

#[derive(Debug, Serialize)]
pub struct SourceFileNode {
    pub path: String,
    pub declarations: Mutex<Vec<Arc<DeclarationNode>>>,
}

impl PartialEq for SourceFileNode {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum AbstractSyntaxNode {
    SourceFile(Arc<SourceFileNode>),
    Declaration(DeclarationNode),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<AbstractSyntaxNode>,
}
