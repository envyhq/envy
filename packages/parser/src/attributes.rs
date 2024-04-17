use crate::abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode, Identifier, Literal};
use std::{sync::Arc, sync::Weak};

#[derive(Debug, Clone)]
pub struct AttributeDeclarationNode {
    pub identifier: Identifier,
    pub value: Literal,
    pub parent: Weak<AbstractSyntaxNode>,
}

#[derive(Debug, Clone)]
pub struct PartialAttributeDeclarationNode {
    pub identifier: Option<Identifier>,
    pub value: Option<Literal>,
}

impl TryFrom<PartialAttributeDeclarationNode> for Arc<AttributeDeclarationNode> {
    type Error = ();

    fn try_from(partial: PartialAttributeDeclarationNode) -> Result<Self, Self::Error> {
        if partial.identifier.is_none() || partial.value.is_none() {
            return Err(());
        }

        Ok(Arc::new(AttributeDeclarationNode {
            parent: Weak::new(),
            identifier: partial.identifier.unwrap(),
            value: partial.value.unwrap(),
        }))
    }
}

impl TryFrom<PartialAttributeDeclarationNode> for AttributeDeclarationNode {
    type Error = ();

    fn try_from(partial: PartialAttributeDeclarationNode) -> Result<Self, Self::Error> {
        if partial.identifier.is_none() || partial.value.is_none() {
            return Err(());
        }

        Ok(AttributeDeclarationNode {
            parent: Weak::new(),
            identifier: partial.identifier.unwrap(),
            value: partial.value.unwrap(),
        })
    }
}

impl From<AttributeDeclarationNode> for PartialAttributeDeclarationNode {
    fn from(partial: AttributeDeclarationNode) -> Self {
        PartialAttributeDeclarationNode {
            identifier: Some(partial.identifier.clone()),
            value: Some(partial.value.clone()),
        }
    }
}

impl From<AbstractSyntaxNode> for PartialAttributeDeclarationNode {
    fn from(partial: AbstractSyntaxNode) -> Self {
        match partial {
            AbstractSyntaxNode::Declaration(declaration) => match declaration {
                DeclarationNode::AttributeDeclaration(attribute) => attribute.into(),
                _ => panic!("Invalid conversion"),
            },
            _ => panic!("Invalid conversion"),
        }
    }
}
