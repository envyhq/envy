use async_trait::async_trait;
use envy_lexer::LexerType;
use envy_parser::{AbstractSyntaxNode, DeclarationNode, VarDeclarationNode};
use envy_provider_resolver::ProviderResolver;
use futures::{stream, StreamExt};
use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
};

#[derive(Debug)]
pub enum ResolutionError {
    ProviderError,
}

#[derive(Debug, Clone)]
pub struct ResolvedValue {
    pub key: String,
    pub value: Option<Vec<u8>>,
    pub deserialized_value: Option<DeserializedValue>,
    pub provider: String,
}

#[derive(Debug, Clone)]
pub enum DeserializedValue {
    String(String),
    Int(i32),
    Float(f32),
    Url(String),
}

impl Display for DeserializedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeserializedValue::String(s) => write!(f, "{}", s),
            DeserializedValue::Int(i) => write!(f, "{}", i),
            DeserializedValue::Float(fl) => write!(f, "{}", fl),
            DeserializedValue::Url(u) => write!(f, "{}", u),
        }
    }
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
}

#[derive(Debug, Default, Clone)]
pub struct VarResolver {
    pub provider_resolver: ProviderResolver,
}

#[async_trait]
impl TreeResolver for VarResolver {
    async fn resolve_var(
        &self,
        node: &VarDeclarationNode,
    ) -> Result<ResolvedValue, ResolutionError> {
        let provider = self
            .provider_resolver
            .resolve_node(&AbstractSyntaxNode::Declaration(
                DeclarationNode::VarDeclaration(Arc::new(node.clone())),
            ))
            .unwrap();

        let value = provider
            .get_value(&node.identifier.value)
            .await
            .map_err(|_| ResolutionError::ProviderError)?;

        let string_value = String::from_utf8(value.clone()).unwrap();

        let deserialized_value = match node.type_value.value {
            LexerType::String => Some(DeserializedValue::String(string_value)),
            LexerType::Int => string_value.parse::<i32>().ok().map(DeserializedValue::Int),
            LexerType::Float => string_value
                .parse::<f32>()
                .ok()
                .map(DeserializedValue::Float),
            LexerType::Url => Some(DeserializedValue::Url(string_value)),
        };

        Ok(ResolvedValue {
            key: node.identifier.value.clone(),
            value: Some(value),
            deserialized_value,
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
            .filter_map(|d: &Result<Vec<ResolvedValue>, ResolutionError>| {
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
}
