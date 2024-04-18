use nv_parser::{AbstractSyntaxNode, DeclarationNode, ProviderDeclarationNode};
pub use nv_provider_core::Provider;
use std::sync::Arc;

pub type ResolverProvider = Arc<dyn Provider + Sync + Send>;

#[derive(Debug, Default, Clone)]
pub struct ProviderResolver {
    pub providers: Vec<ResolverProvider>,
}

impl ProviderResolver {
    pub fn resolve(&self, node: &AbstractSyntaxNode) -> Vec<ResolverProvider> {
        let nodes = self.find_provider_nodes(node);

        for node in nodes {
            println!("Adding provider: {:?}", node);
        }

        vec![]
    }

    fn find_provider_nodes(&self, node: &AbstractSyntaxNode) -> Vec<ProviderDeclarationNode> {
        match node {
            AbstractSyntaxNode::SourceFile(source_file) => {
                let lock = source_file.declarations.lock().unwrap();
                lock.clone()
                    .into_iter()
                    .filter_map(|d| match d.as_ref() {
                        DeclarationNode::ProviderDeclaration(provider) => Some(provider.clone()),
                        _ => None,
                    })
                    .collect()
            }
            AbstractSyntaxNode::Declaration(declaration) => {
                if let DeclarationNode::ProviderDeclaration(provider) = declaration {
                    vec![provider.clone()]
                } else {
                    vec![]
                }
            }
        }
    }
}
