use std::sync::Arc;

use async_trait::async_trait;
use futures::{stream, StreamExt};
use nv_parser::{AbstractSyntaxNode, AbstractSyntaxTree, DeclarationNode, VarDeclarationNode};
pub use nv_provider_core::Provider;

#[derive(Debug)]
pub enum ResolutionError {
    ProviderError,
}

#[derive(Debug, Clone)]
pub struct ResolvedValue {
    pub key: String,
    pub value: Option<String>,
}

#[async_trait]
pub trait TreeResolver {
    async fn resolve_var(&self, ast: VarDeclarationNode) -> Result<ResolvedValue, ResolutionError>;
    async fn resolve_declaration(
        &self,
        ast: DeclarationNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError>;
    async fn resolve_node(
        &self,
        ast: AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError>;
    async fn resolve(&self, ast: AbstractSyntaxTree)
        -> Result<Vec<ResolvedValue>, ResolutionError>;
}

pub type ResolverProvider = Arc<dyn Provider + Sync + Send>;

pub struct Resolver {
    pub providers: Vec<ResolverProvider>,
}

#[async_trait]
impl TreeResolver for Resolver {
    async fn resolve_var(
        &self,
        node: VarDeclarationNode,
    ) -> Result<ResolvedValue, ResolutionError> {
        let provider = self.providers.first().unwrap();

        let value = provider.get_value(&node.identifier.clone()).await;

        Ok(ResolvedValue {
            key: node.identifier,
            value: value.ok(),
        })
    }

    async fn resolve_declaration(
        &self,
        node: DeclarationNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        match node {
            DeclarationNode::VarDeclaration(var) => Ok(vec![self.resolve_var(var).await?]),
            DeclarationNode::ModuleDeclaration(module) => Ok(stream::iter(module.declarations)
                .map(|d| async move { self.resolve_declaration(d.clone()).await })
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
            _ => Err(ResolutionError::ProviderError),
        }
    }

    async fn resolve_node(
        &self,
        node: AbstractSyntaxNode,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        match node {
            AbstractSyntaxNode::SourceFile(source_file) => {
                Ok(stream::iter(source_file.declarations)
                    .map(|d| async move { self.resolve_declaration(d.clone()).await })
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
            AbstractSyntaxNode::Declaration(declaration) => {
                self.resolve_declaration(declaration).await
            }
        }
    }

    async fn resolve(
        &self,
        ast: AbstractSyntaxTree,
    ) -> Result<Vec<ResolvedValue>, ResolutionError> {
        let mut resolved_values = vec![];

        if let Some(root) = ast.root {
            resolved_values = self.resolve_node(root).await?;
        }

        Ok(resolved_values)
    }
}
