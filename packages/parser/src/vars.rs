use nv_lexer::{LexerType, LexerVarModifierKeyword};

use crate::attributes::{AttributeDeclarationNode, PartialAttributeDeclarationNode};

#[derive(Debug, Clone)]
pub struct VarDeclarationNode {
    pub identifier: String,
    pub type_value: LexerType,
    pub modifier: Option<LexerVarModifierKeyword>,
    pub attributes: Vec<AttributeDeclarationNode>,
}

#[derive(Debug, Clone)]
pub struct PartialVarDeclarationNode {
    pub identifier: Option<String>,
    pub type_value: Option<LexerType>,
    pub modifier: Option<LexerVarModifierKeyword>,
    pub attributes: Vec<PartialAttributeDeclarationNode>,
}

impl TryFrom<PartialVarDeclarationNode> for VarDeclarationNode {
    type Error = ();

    fn try_from(partial: PartialVarDeclarationNode) -> Result<Self, Self::Error> {
        if partial.identifier.is_none() || partial.type_value.is_none() {
            return Err(());
        }

        Ok(VarDeclarationNode {
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
