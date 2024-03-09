use crate::abstract_syntax_tree::{AbstractSyntaxNode, DeclarationNode};
use nv_lexer::tokens::LexerLiteral;

#[derive(Debug, Clone)]
pub struct AttributeDeclarationNode {
    pub identifier: String,
    pub value: LexerLiteral,
}

#[derive(Debug, Clone)]
pub struct PartialAttributeDeclarationNode {
    pub identifier: Option<String>,
    pub value: Option<LexerLiteral>,
}

impl TryFrom<PartialAttributeDeclarationNode> for AttributeDeclarationNode {
    type Error = ();

    fn try_from(partial: PartialAttributeDeclarationNode) -> Result<Self, Self::Error> {
        if partial.identifier.is_none() || partial.value.is_none() {
            return Err(());
        }

        Ok(AttributeDeclarationNode {
            identifier: partial.identifier.unwrap(),
            value: partial.value.unwrap(),
        })
    }
}

impl From<AttributeDeclarationNode> for PartialAttributeDeclarationNode {
    fn from(partial: AttributeDeclarationNode) -> Self {
        PartialAttributeDeclarationNode {
            identifier: Some(partial.identifier),
            value: Some(partial.value),
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
