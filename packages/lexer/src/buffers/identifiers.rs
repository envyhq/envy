use crate::{
    tokens::{LexerDeclarationKeyword, LexerVarModifierKeyword},
    LexerKeyword,
};
use strum::IntoEnumIterator;

pub fn buffer_to_keyword(buffer: &Vec<String>) -> Option<LexerKeyword> {
    let buffered = buffer.join("");

    let modifier = LexerVarModifierKeyword::iter()
        .find(|m| m.to_string() == buffered)
        .map(|m| LexerKeyword::VarModifierKeyword(m));

    if modifier.is_some() {
        return modifier;
    }

    LexerDeclarationKeyword::iter()
        .find(|m| m.to_string() == buffered)
        .map(|m| LexerKeyword::DeclarationKeyword(m))
}
