use crate::Parser;

pub struct AttributeBlockParser {}

impl Parser for AttributeBlockParser {
    fn parse(&mut self) -> usize {
        0
    }
}

impl AttributeBlockParser {
    pub fn new() -> Self {
        Self {}
    }
}
