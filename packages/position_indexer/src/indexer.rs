use nv_parser::{AbstractSyntaxNode, DeclarationNode, TokenPosition};
use std::{collections::HashMap, sync::Weak};

pub type PositionIndex = HashMap<TokenPosition, Weak<AbstractSyntaxNode>>;

pub struct Indexer;

impl Indexer {
    pub fn index_node(node: Weak<AbstractSyntaxNode>) -> PositionIndex {
        let mut index = HashMap::new();

        let node_val = node.upgrade().unwrap();

        println!("index node: {:#?}", node_val);

        match node_val.as_ref() {
            AbstractSyntaxNode::SourceFile(source) => {
                println!(
                    "wowow source {:#?}",
                    source.declarations.lock().unwrap().len()
                );
                source
                    .declarations
                    .lock()
                    .unwrap()
                    .iter()
                    .for_each(|declaration| match declaration.as_ref() {
                        DeclarationNode::VarDeclaration(var) => {
                            println!("index var 1: {:#?}", var);
                            for pos in
                                var.identifier.range.from.column..var.identifier.range.to.column
                            {
                                index.insert(
                                    TokenPosition::new(var.identifier.range.from.line, pos),
                                    node.clone(),
                                );
                            }
                        }
                        _ => {}
                    })
            }
            AbstractSyntaxNode::Declaration(declaration) => match declaration {
                DeclarationNode::VarDeclaration(var) => {
                    println!("index var 2: {:#?}", var);
                    index.insert(var.identifier.range.from.clone(), node.clone());
                }
                _ => {}
            },
        }

        println!("indexing finnnnn {:#?}", index);

        index
    }
}
