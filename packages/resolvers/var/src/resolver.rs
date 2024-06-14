use async_trait::async_trait;
use futures::{stream, StreamExt};
use nv_parser::{AbstractSyntaxNode, DeclarationNode, VarDeclarationNode};
pub use nv_provider_core::Provider;
use nv_provider_resolver::ProviderResolver;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ResolutionError {
    ProviderError,
}

#[derive(Debug, Clone)]
pub struct ResolvedValue {
    pub key: String,
    pub value: Option<Vec<u8>>,
    pub provider: String,
}

impl Error for ResolutionError {}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error resolving value")
    }
}

#[async_trait]
pub trait TreeResolver {
    async fn resolve_var(&self, ast: &VarDeclarationNode)
        -> Result<ResolvedValue, ResolutionError>;
    async fn resolve_declaration(
        &self,
        ast: &DeclarationNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError>;
    async fn resolve_node(
        &self,
        ast: &AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError>;
    async fn resolve(
        &self,
        ast: &AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError>;
}

#[derive(Debug, Default, Clone)]
pub struct VarResolver {
    pub provider_resolver: ProviderResolver,
}

impl VarResolver {
    pub fn init(&mut self, node: &AbstractSyntaxNode) {
        self.provider_resolver.resolve(node);
    }
}

#[async_trait]
impl TreeResolver for VarResolver {
    async fn resolve_var(
        &self,
        node: &VarDeclarationNode,
    ) -> Result<ResolvedValue, ResolutionError> {
        // TODO: Should use something like self.provider_resolver.providers.find(|p| p.identifier.value == node.provider).unwrap();...
        // Need to link var nodes to their provider nodes, then resolve the provider, then resolve the var
        // And then should rly make self.providers a hashmap across all resolver types
        let provider = self.provider_resolver.providers.first().unwrap();

        let value = provider.get_value(&node.identifier.value).await;

        Ok(ResolvedValue {
            key: node.identifier.value.clone(),
            value: value.ok(),
            provider: provider.name().to_owned(),
        })
    }

    async fn resolve_declaration(
        &self,
        node: &DeclarationNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        match node {
            DeclarationNode::VarDeclaration(var) => Ok(vec![self.resolve_var(var).await?]),
            DeclarationNode::ModuleDeclaration(module) => {
                Ok(stream::iter(module.declarations.clone())
                    .map(|d| async move { self.resolve_declaration(d.as_ref()).await })
                    .buffer_unordered(5)
                    .collect::<Vec<_>>()
                    .await
                    .iter()
                    .filter_map(|d| {
                        if d.is_ok() {
                            Some(d.as_ref().unwrap().clone())
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect::<Vec<_>>())
            }
            _ => Err(ResolutionError::ProviderError),
        }
    }

    async fn resolve_node(
        &self,
        node: &AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        match node {
            AbstractSyntaxNode::SourceFile(source_file) => Ok(stream::iter({
                let lock = source_file.declarations.lock().unwrap();
                lock.clone().into_iter()
            })
            .map(|d| async move { self.resolve_declaration(d.as_ref()).await })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await
            .iter()
            .filter_map(|d| {
                if d.is_ok() {
                    Some(d.as_ref().unwrap().clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>()),
            AbstractSyntaxNode::Declaration(declaration) => {
                self.resolve_declaration(declaration).await
            }
        }
    }

    async fn resolve(
        &self,
        ast: &AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        let resolved_values = self.resolve_node(ast).await?;

        Ok(resolved_values)
    }
}
