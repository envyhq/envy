use std::{collections::HashMap, fs};

use envy_parser::SourceFileParser;
use envy_var_resolver::{DeserializedValue, ResolutionError, TreeResolver, VarResolver};

#[derive(Debug)]
pub struct BuildOpts<'a> {
    pub force: &'a bool,
    pub target_config: &'a str,
}

pub async fn build(opts: BuildOpts<'_>) -> Result<(), ResolutionError> {
    println!("Building opts:{:?} isForce: {}", opts, opts.force);

    let (_, ast) = SourceFileParser::from_file(opts.target_config);

    let resolver = VarResolver::default();
    let resolved = resolver.resolve_node(ast.as_ref()).await?;

    let target_config_filename = opts.target_config.split("/").last().unwrap();
    let output_path = format!("./.nv/{}x", target_config_filename);

    let values_file: HashMap<String, DeserializedValue> = resolved
        .iter()
        .map(|resolved| {
            (
                resolved.key.clone(),
                resolved.deserialized_value.clone().unwrap().clone(),
            )
        })
        .collect();

    let contents = values_file
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    let _ = fs::write(output_path.clone(), contents.clone());

    log::info!("resolved:\n{:#?}", resolved);
    log::info!("output_path:\n{:#?}", output_path);
    log::info!("values_file:\n{:#?}", values_file);
    log::info!("contents:\n{:#?}", contents);

    Ok(())
}
