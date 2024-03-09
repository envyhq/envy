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

impl TryInto<VarDeclarationNode> for PartialVarDeclarationNode {
    type Error = ();

    fn try_into(self) -> Result<VarDeclarationNode, Self::Error> {
        if self.identifier.is_none() || self.type_value.is_none() {
            return Err(());
        }

        Ok(VarDeclarationNode {
            identifier: self.identifier.unwrap(),
            type_value: self.type_value.unwrap(),
            modifier: self.modifier,
            attributes: self
                .attributes
                .into_iter()
                .map(|attr| attr.try_into().unwrap())
                .collect(),
        })
    }
}
