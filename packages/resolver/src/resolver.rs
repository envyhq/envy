use nv_parser::{AbstractSyntaxNode, AbstractSyntaxTree, DeclarationNode, VarDeclarationNode};

pub use nv_provider_core::Provider;

#[derive(Debug)]
pub struct ResolvedValue {
    pub key: String,
    pub value: Option<String>,
}

pub trait TreeResolver {
    fn resolve_var(&self, ast: VarDeclarationNode) -> ResolvedValue;
    fn resolve_declaration(&self, ast: DeclarationNode) -> Vec<ResolvedValue>;
    fn resolve_node(&self, ast: AbstractSyntaxNode) -> Vec<ResolvedValue>;
    fn resolve(&self, ast: AbstractSyntaxTree) -> Vec<ResolvedValue>;
}

pub struct Resolver {
    pub providers: Vec<Box<dyn Provider>>,
}

impl TreeResolver for Resolver {
    fn resolve_var(&self, node: VarDeclarationNode) -> ResolvedValue {
        let provider = self.providers.iter().find(|p| p.name() == "env").unwrap();

        let value = provider.get_value(node.identifier.clone());

        ResolvedValue {
            key: node.identifier,
            value: value.ok(),
        }
    }

    fn resolve_declaration(&self, node: DeclarationNode) -> Vec<ResolvedValue> {
        match node {
            DeclarationNode::VarDeclaration(var) => vec![self.resolve_var(var)],
            DeclarationNode::ModuleDeclaration(module) => module
                .declarations
                .iter()
                .map(|d| self.resolve_declaration(d.clone()))
                .flatten()
                .collect(),
            _ => unimplemented!(),
        }
    }

    fn resolve_node(&self, node: AbstractSyntaxNode) -> Vec<ResolvedValue> {
        match node {
            AbstractSyntaxNode::SourceFile(source_file) => source_file
                .declarations
                .iter()
                .map(|d| self.resolve_declaration(d.clone()))
                .flatten()
                .collect(),
            AbstractSyntaxNode::Declaration(declaration) => self.resolve_declaration(declaration),
            _ => unimplemented!(),
        }
    }

    fn resolve(&self, ast: AbstractSyntaxTree) -> Vec<ResolvedValue> {
        let mut resolved_values = vec![];

        if let Some(root) = ast.root {
            resolved_values = self.resolve_node(root);
        }

        resolved_values
    }
}
