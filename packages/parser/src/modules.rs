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

pub enum ModuleDeclarationConvertError {
    MissingIdentifier,
}

impl TryFrom<PartialModuleDeclarationNode> for ModuleDeclarationNode {
    type Error = ModuleDeclarationConvertError;

    fn try_from(partial: PartialModuleDeclarationNode) -> Result<Self, Self::Error> {
        let identifier = partial
            .identifier
            .ok_or(ModuleDeclarationConvertError::MissingIdentifier)?;

        Ok(ModuleDeclarationNode {
            identifier,
            declarations: partial
                .declarations
                .iter()
                .map(|d| Arc::new(d.clone()))
                .collect(),
        })
    }
}
