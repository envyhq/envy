use crate::{attributes::AttributeDeclarationNode, vars::VarDeclarationNode};

#[derive(Debug, Clone)]
pub enum DeclarationNode {
    VarDeclaration(VarDeclarationNode),
    ModuleDeclaration,
    ProviderDeclaration,
    AttributeDeclaration(AttributeDeclarationNode),
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
