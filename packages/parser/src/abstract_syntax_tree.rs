use crate::{
    attributes::AttributeDeclarationNode, modules::ModuleDeclarationNode,
    providers::ProviderDeclarationNode, vars::VarDeclarationNode,
};
use nv_lexer::{
    tokens::{LexerLiteral, TokenRange},
    LexerType, LexerVarModifierKeyword,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<V> {
    pub value: V,
    pub range: TokenRange,
}
impl<V> Leaf<V> {
    pub fn new(value: V, range: TokenRange) -> Self {
        Self { value, range }
    }
}

pub type Identifier = Leaf<String>;
pub type Literal = Leaf<LexerLiteral>;
pub type Type = Leaf<LexerType>;
pub type ProviderType = Leaf<String>;
pub type Modifier = Leaf<LexerVarModifierKeyword>;

#[derive(Debug, Clone)]
pub enum DeclarationNode {
    VarDeclaration(VarDeclarationNode),
    ModuleDeclaration(ModuleDeclarationNode),
    ProviderDeclaration(ProviderDeclarationNode),
    AttributeDeclaration(AttributeDeclarationNode),
}

impl From<AbstractSyntaxNode> for DeclarationNode {
    fn from(declaration: AbstractSyntaxNode) -> Self {
        match declaration {
            AbstractSyntaxNode::Declaration(declaration) => match declaration {
                DeclarationNode::VarDeclaration(var) => var.clone().into(),
                _ => panic!("Invalid conversion"),
            },
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<VarDeclarationNode> for DeclarationNode {
    fn from(declaration: VarDeclarationNode) -> Self {
        DeclarationNode::VarDeclaration(declaration)
    }
}

#[derive(Debug)]
pub struct SourceFileNode {
    pub declarations: Mutex<Vec<Arc<DeclarationNode>>>,
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxNode {
    SourceFile(Arc<SourceFileNode>),
    Declaration(DeclarationNode),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<AbstractSyntaxNode>,
}
