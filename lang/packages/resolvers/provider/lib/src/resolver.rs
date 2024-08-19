use envy_parser::{AbstractSyntaxNode, DeclarationNode, ProviderDeclarationNode};
use envy_provider_core::{Provider, ProviderError};
use envy_provider_env::EnvProvider;
use std::sync::Arc;

pub type ResolverProvider = Arc<dyn Provider + Sync + Send>;

#[derive(Debug, Default, Clone)]
pub struct ProviderResolver {
    pub providers: Vec<ResolverProvider>,
}

impl ProviderResolver {
    pub fn resolve_node(
        &self,
        ast: &AbstractSyntaxNode,
    ) -> Result<ResolverProvider, ProviderError> {
        let nodes = self.find_provider_nodes(ast);

        for node in nodes {
            println!("Adding provider: {:?}", node);
        }

        Ok(Arc::new(EnvProvider::default()))
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
