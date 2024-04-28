use serde::Serialize;

use crate::{
    abstract_syntax_tree::{Identifier, ProviderType},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ProviderDeclarationNode {
    pub identifier: Identifier,
    pub type_value: ProviderType,
    pub attributes: Vec<AttributeDeclarationNode>,
}

#[derive(Debug, Clone)]
pub struct PartialProviderDeclarationNode {
    pub identifier: Option<Identifier>,
    pub type_value: Option<ProviderType>,
    pub attributes: Vec<PartialAttributeDeclarationNode>,
}

#[derive(Debug)]
pub enum ProviderDeclarationConvertError {
    MissingIdentifier,
    MissingTypeValue,
}

impl TryFrom<PartialProviderDeclarationNode> for ProviderDeclarationNode {
    type Error = ProviderDeclarationConvertError;

    fn try_from(partial: PartialProviderDeclarationNode) -> Result<Self, Self::Error> {
        let identifier = partial
            .identifier
            .ok_or(ProviderDeclarationConvertError::MissingIdentifier)?;
        let type_value = partial
            .type_value
            .ok_or(ProviderDeclarationConvertError::MissingTypeValue)?;

        Ok(ProviderDeclarationNode {
            identifier,
            type_value,
            attributes: partial
                .attributes
                .clone()
                .into_iter()
                .filter_map(|attribute| attribute.try_into().ok())
                .collect(),
        })
    }
}
