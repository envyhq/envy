use crate::{
    tokens::{LexerDeclarationKeyword, LexerVarModifierKeyword},
    LexerKeyword,
};
use strum::IntoEnumIterator;

pub fn buffer_to_keyword(buffer: &[String]) -> Option<LexerKeyword> {
    let buffered = buffer.join("");

    let modifier = LexerVarModifierKeyword::iter()
        .find(|m| m.to_string() == buffered)
        .map(LexerKeyword::VarModifierKeyword);

    if modifier.is_some() {
        return modifier;
    }

    LexerDeclarationKeyword::iter()
        .find(|m| m.to_string() == buffered)
        .map(LexerKeyword::DeclarationKeyword)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_none_for_empty_buffer() {
        let buffer = vec![];
        let result = buffer_to_keyword(&buffer);
        assert!(result.is_none());
    }

    #[test]
    fn matches_none_for_unknown_keyword() {
        let buffer = vec!["unknown".to_string()];
        let result = buffer_to_keyword(&buffer);
        assert!(result.is_none());
    }

    #[test]
    fn matches_var_modifier_keyword() {
        let buffer = vec!["pub".to_string()];
        let result = buffer_to_keyword(&buffer);
        assert_eq!(
            result,
            Some(LexerKeyword::VarModifierKeyword(
                LexerVarModifierKeyword::Pub
            ))
        );
    }

    #[test]
    fn matches_provider_declaration_keyword() {
        let buffer = vec!["provider".to_string()];
        let result = buffer_to_keyword(&buffer);
        assert_eq!(
            result,
            Some(LexerKeyword::DeclarationKeyword(
                LexerDeclarationKeyword::Provider
            ))
        );
    }

    #[test]
    fn matches_module_declaration_keyword() {
        let buffer = vec!["module".to_string()];
        let result = buffer_to_keyword(&buffer);
        assert_eq!(
            result,
            Some(LexerKeyword::DeclarationKeyword(
                LexerDeclarationKeyword::Module
            ))
        );
    }

    #[test]
    fn matches_var_declaration_keyword() {
        let buffer = vec!["var".to_string()];
        let result = buffer_to_keyword(&buffer);
        assert_eq!(
            result,
            Some(LexerKeyword::DeclarationKeyword(
                LexerDeclarationKeyword::Var
            ))
        );
    }
}
