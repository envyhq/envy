use envy_parser::{AbstractSyntaxNode, DeclarationNode, ProviderDeclarationNode};
use envy_provider_core::Provider;
use std::sync::Arc;

pub type ResolverProvider = Arc<dyn Provider + Sync + Send>;

#[derive(Debug, Default, Clone)]
pub struct ProviderResolver {
    pub providers: Vec<ResolverProvider>,
}

impl ProviderResolver {
    pub fn resolve(&self, node: &AbstractSyntaxNode) {
        let nodes = self.find_provider_nodes(node);

        for node in nodes {
            println!("Adding provider: {:?}", node);
            // TODO: node is a provider declaration node, we need to create a real provider from
            // it like Arc::new(EnvProvider {}) for example..
            // then we add that provider to self.providers and return it
            // this actually involves potentially downloading provider from net
            // but... we should by default throw an error if its not already available locally?
            // note that EnvProvider needs to be updated to be a client of the EnvProvider under process model
            // we can probably wrap EnvProvider etc in a ProviderServer struct of some kind that
            // that manages the IPC protocol + calling in to the underlying EnvProvider etc
        }
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
