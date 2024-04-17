use crate::abstract_syntax_tree::DeclarationNode;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ModuleDeclarationNode {
    pub identifier: String,
    pub declarations: Vec<Arc<DeclarationNode>>,
}

#[derive(Debug, Clone)]
pub struct PartialModuleDeclarationNode {
    pub identifier: Option<String>,
    pub declarations: Vec<DeclarationNode>,
}

impl From<PartialModuleDeclarationNode> for ModuleDeclarationNode {
    fn from(partial: PartialModuleDeclarationNode) -> Self {
        ModuleDeclarationNode {
            identifier: partial.identifier.unwrap(),
            declarations: partial
                .declarations
                .iter()
                .map(|d| Arc::new(d.clone()))
                .collect(),
        }
    }
}
