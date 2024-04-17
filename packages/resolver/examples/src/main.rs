use futures::executor;
use nv_lexer::{Lexer, SourceFileLexer};
use nv_parser::{AbstractSyntaxNode, SourceFileParser};
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

    let mut lexer = SourceFileLexer::new(&input);
    lexer.lex();

    let (processed_count, ast) = SourceFileParser::parse(&lexer.tokens);

    log::info!("ast processed_count: {}", processed_count);

    let env: ResolverProvider = Arc::new(EnvProvider {});
    let aws_sm: ResolverProvider = Arc::new(AwsSecretsManagerProvider {});

    let mut available_providers: HashMap<String, ResolverProvider> = HashMap::new();
    available_providers.insert("env".to_string(), env);
    available_providers.insert("aws_sm".to_string(), aws_sm);

    let providers: Vec<ResolverProvider> = if let AbstractSyntaxNode::SourceFile(source) =
        ast.as_ref().clone()
    {
        source
            .declarations
            .lock()
            .unwrap()
            .iter()
            .filter_map(|d| match d.as_ref() {
                nv_parser::DeclarationNode::ProviderDeclaration(declaration)
                    if available_providers.contains_key(declaration.type_value.value.as_str()) =>
                {
                    let provider: ResolverProvider = available_providers
                        .get(declaration.type_value.value.as_str())
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

    let resolved = resolver.resolve(ast.as_ref().clone()).await;

    log::info!("{:#?}", resolved);

    Ok(())
}

fn main() {
    let _ = executor::block_on(async_main());
}
