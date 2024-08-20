mod gen;

use r#gen::{parse_config, Config};
use std::collections::HashMap;
use tokio::fs;

async fn load_config(path_input: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let path = path_input.unwrap_or(".nv/config.nvx");
    let contents = fs::read_to_string(path).await?;
    Ok(contents)
}

async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let config_name = "../../lang/packages/cli/.nv/simple-vars.nvx";
    let result = load_config(Some(config_name)).await?;

    let mut config_map: HashMap<String, String> = HashMap::new();
    for line in result.lines() {
        if let Some((key, value)) = line.split_once(':') {
            config_map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    let parsed_config: Config = parse_config(&config_map).unwrap();

    println!(
        "my_cool_var: {}\nmy_other_var: {}",
        config_map
            .get("my_cool_var")
            .unwrap_or(&"Not found".to_string()),
        config_map
            .get("my_other_var")
            .unwrap_or(&"Not found".to_string())
    );

    println!(
        "Parsed Config: {:?}\nmy_cool_var: {}\nmy_other_var: {}",
        parsed_config, parsed_config.my_cool_var, parsed_config.my_other_var
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = test().await {
        eprintln!("Error: {}", e);
    }
}
