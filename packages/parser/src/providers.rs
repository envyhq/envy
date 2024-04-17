use crate::{
    abstract_syntax_tree::{Identifier, ProviderType},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
};

#[derive(Debug, Clone)]
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

impl From<PartialProviderDeclarationNode> for ProviderDeclarationNode {
    fn from(partial: PartialProviderDeclarationNode) -> Self {
        ProviderDeclarationNode {
            identifier: partial.identifier.clone().unwrap(),
            type_value: partial.type_value.clone().unwrap(),
            attributes: partial
                .attributes
                .clone()
                .into_iter()
                .filter_map(|attribute| attribute.try_into().map_or(None, |a| Some(a)))
                .collect(),
        }
    }
}
