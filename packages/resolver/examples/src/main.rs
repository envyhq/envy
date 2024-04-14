use futures::executor;
use nv_parser::{Parser, SourceFileParser};
use nv_provider_aws_secrets_manager::AwsSecretsManagerProvider;
use nv_provider_env::EnvProvider;
use nv_resolver::{Resolver, ResolverProvider, TreeResolver};
use std::{collections::HashMap, env, fs, sync::Arc};

async fn async_main() -> Result<(), ()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut input = "var my_cool_var: string\nvar my_other_var: int".to_string();

    if args.len() > 1 && args[1] == "--file" && !args[2].is_empty() {
        let file = args[2].to_string();
        input = fs::read_to_string(file).unwrap();
    }

    let mut parser = SourceFileParser::new(input);

    let processed_count = parser.parse();

    log::info!("ast processed_count: {}", processed_count);

    let env: ResolverProvider = Arc::new(EnvProvider {});
    let aws_sm: ResolverProvider = Arc::new(AwsSecretsManagerProvider {});

    let mut available_providers: HashMap<String, ResolverProvider> = HashMap::new();
    available_providers.insert("env".to_string(), env);
    available_providers.insert("aws_sm".to_string(), aws_sm);

    let providers: Vec<ResolverProvider> =
        if let Some(nv_parser::AbstractSyntaxNode::SourceFile(source)) = parser.ast.clone().root {
            source
                .declarations
                .iter()
                .filter_map(|d| match d {
                    nv_parser::DeclarationNode::ProviderDeclaration(declaration)
                        if available_providers.contains_key(declaration.type_value.as_str()) =>
                    {
                        let provider: ResolverProvider = available_providers
                            .get(declaration.type_value.as_str())
                            .unwrap()
                            .clone();

                        Some(provider)
                    }
                    _ => None,
                })
                .collect::<Vec<ResolverProvider>>()
        } else {
            vec![]
        };

    let resolver = Resolver { providers };

    let resolved = resolver.resolve(parser.ast).await;

    log::info!("{:#?}", resolved);

    Ok(())
}

fn main() {
    let _ = executor::block_on(async_main());
}
