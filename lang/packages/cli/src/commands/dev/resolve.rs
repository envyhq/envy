use envy_parser::SourceFileParser;
use envy_var_resolver::{ResolutionError, ResolvedValue, TreeResolver, VarResolver};

pub async fn resolve(target_config: &str) -> Result<Vec<ResolvedValue>, ResolutionError> {
    let (_, ast) = SourceFileParser::from_file(target_config);

    let resolver = VarResolver::default();

    let resolved = resolver.resolve_node(ast.as_ref()).await;

    log::info!("resolved:\n{:#?}\n", resolved);

    resolved
}
