use crate::Parser;

pub struct ProviderDeclarationParser {}

impl Parser for ProviderDeclarationParser {
    fn parse(&mut self) -> usize {
        0
    }
}

impl ProviderDeclarationParser {
    pub fn new() -> Self {
        Self {}
    }
}
