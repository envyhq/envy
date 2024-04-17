use nv_parser::{AbstractSyntaxNode, DeclarationNode, TokenPosition, VarDeclarationNode};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

pub type PositionIndex = HashMap<TokenPosition, Weak<VarDeclarationNode>>;

// This is an experimental token position indexer.
// It creates a hash map of all possible cursor positions to the AST node that exists there.
// This is used by the language server to very quickly deliver results based on queries including
// the position of the cursor. For example, hovering a variable name.
// It is an abuse of RAM for large projects but I'd like to benchmark it.
#[derive(Debug, Clone, Default)]
pub struct Indexer {
    pub position_index: PositionIndex,
}

impl Indexer {
    pub fn node_at(&self, position: &TokenPosition) -> Option<&Weak<VarDeclarationNode>> {
        self.position_index.get(position)
    }

    pub fn index(&mut self, node: &Arc<AbstractSyntaxNode>) {
        self.position_index = HashMap::new();

        match node.as_ref() {
            AbstractSyntaxNode::SourceFile(source) => source
                .declarations
                .lock()
                .unwrap()
                .iter()
                .for_each(|declaration| {
                    if let DeclarationNode::VarDeclaration(var) = declaration.as_ref() {
                        for pos in var.identifier.range.from.column..var.identifier.range.to.column
                        {
                            self.position_index.insert(
                                TokenPosition::new(var.identifier.range.from.line, pos),
                                Arc::downgrade(var),
                            );
                        }
                    }
                }),
            AbstractSyntaxNode::Declaration(declaration) => {
                if let DeclarationNode::VarDeclaration(var) = declaration {
                    for pos in var.identifier.range.from.column..var.identifier.range.to.column {
                        self.position_index.insert(
                            TokenPosition::new(var.identifier.range.from.line, pos),
                            Arc::downgrade(var),
                        );
                    }
                }
            }
        };
    }
}
