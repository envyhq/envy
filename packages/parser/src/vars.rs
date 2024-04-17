use crate::{
    abstract_syntax_tree::{Identifier, Modifier, Type},
    attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode},
    AbstractSyntaxNode,
};
use std::{sync::Arc, sync::Weak};

#[derive(Debug, Clone)]
pub struct VarDeclarationNode {
    pub identifier: Identifier,
    pub type_value: Type,
    pub modifier: Option<Modifier>,
    pub attributes: Vec<Arc<AttributeDeclarationNode>>,
    pub parent: Weak<AbstractSyntaxNode>,
}

#[derive(Debug, Clone)]
pub struct PartialVarDeclarationNode {
    pub identifier: Option<Identifier>,
    pub type_value: Option<Type>,
    pub modifier: Option<Modifier>,
    pub attributes: Vec<PartialAttributeDeclarationNode>,
    pub parent: Weak<AbstractSyntaxNode>,
}

impl TryFrom<PartialVarDeclarationNode> for VarDeclarationNode {
    type Error = ();

    fn try_from(partial: PartialVarDeclarationNode) -> Result<Self, Self::Error> {
        if partial.identifier.is_none() || partial.type_value.is_none() {
            return Err(());
        }

        Ok(VarDeclarationNode {
            parent: partial.parent,
            identifier: partial.identifier.unwrap(),
            type_value: partial.type_value.unwrap(),
            modifier: partial.modifier,
            attributes: partial
                .attributes
                .into_iter()
                .filter_map(|attribute| attribute.try_into().map_or(None, |a| Some(a)))
                .collect(),
        })
    }
}
