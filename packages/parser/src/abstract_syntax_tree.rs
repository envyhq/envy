use crate::{
    attributes::AttributeDeclarationNode, modules::ModuleDeclarationNode,
    providers::ProviderDeclarationNode, vars::VarDeclarationNode,
};

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
                DeclarationNode::VarDeclaration(var) => var.into(),
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

#[derive(Debug, Clone)]
pub struct SourceFileNode {
    pub declarations: Vec<DeclarationNode>,
}

#[derive(Debug, Clone)]
pub enum AbstractSyntaxNode {
    SourceFile(SourceFileNode),
    Declaration(DeclarationNode),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<AbstractSyntaxNode>,
}
