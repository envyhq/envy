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

impl TryInto<AttributeDeclarationNode> for PartialAttributeDeclarationNode {
    type Error = ();

    fn try_into(self) -> Result<AttributeDeclarationNode, Self::Error> {
        if self.identifier.is_none() || self.value.is_none() {
            return Err(());
        }

        Ok(AttributeDeclarationNode {
            identifier: self.identifier.unwrap(),
            value: self.value.unwrap(),
        })
    }
}
