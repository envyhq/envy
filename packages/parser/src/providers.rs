use crate::attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode};

#[derive(Debug, Clone)]
pub struct ProviderDeclarationNode {
    pub identifier: String,
    pub type_value: String,
    pub attributes: Vec<AttributeDeclarationNode>,
}

#[derive(Debug, Clone)]
pub struct PartialProviderDeclarationNode {
    pub identifier: Option<String>,
    pub type_value: Option<String>,
    pub attributes: Vec<PartialAttributeDeclarationNode>,
}

impl From<PartialProviderDeclarationNode> for ProviderDeclarationNode {
    fn from(partial: PartialProviderDeclarationNode) -> Self {
        ProviderDeclarationNode {
            identifier: partial.identifier.unwrap(),
            type_value: partial.type_value.unwrap(),
            attributes: partial
                .attributes
                .into_iter()
                .filter_map(|attribute| attribute.try_into().map_or(None, |a| Some(a)))
                .collect(),
        }
    }
}
