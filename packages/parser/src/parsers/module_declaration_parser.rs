use crate::Parser;

pub struct ModuleDeclarationParser {}

impl Parser for ModuleDeclarationParser {
    fn parse(&mut self) -> usize {
        0
    }
}

impl ModuleDeclarationParser {
    pub fn new() -> Self {
        Self {}
    }
}
