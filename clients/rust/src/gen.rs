use std::{collections::HashMap, error::Error};

#[derive(Debug)]
enum ConfigValue {
    String(String),
    Integer(i32),
    Float(f64),
}

impl ConfigValue {
    fn as_string(&self) -> Option<&String> {
        if let ConfigValue::String(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<i32> {
        if let ConfigValue::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    fn as_float(&self) -> Option<f64> {
        if let ConfigValue::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
}

fn parse_string(value: &str) -> ConfigValue {
    ConfigValue::String(value.to_string())
}

fn parse_integer(value: &str) -> ConfigValue {
    ConfigValue::Integer(value.parse::<i32>().expect("Failed to parse integer"))
}

fn parse_float(value: &str) -> ConfigValue {
    ConfigValue::Float(value.parse::<f64>().expect("Failed to parse float"))
}

#[derive(Debug)]
pub struct Config {
    pub my_cool_var: String,
    pub my_other_var: i32,
}

pub fn parse_config(content: &HashMap<String, String>) -> Result<Config, Box<dyn Error>> {
    let parser_config: HashMap<&str, &str> =
        vec![("my_cool_var", "string"), ("my_other_var", "integer")]
            .into_iter()
            .collect();

    let parsers: HashMap<&str, fn(&str) -> ConfigValue> = vec![
        ("string", parse_string as fn(&str) -> ConfigValue),
        ("integer", parse_integer as fn(&str) -> ConfigValue),
        ("float", parse_float as fn(&str) -> ConfigValue),
    ]
    .into_iter()
    .collect();

    let parsed_config: Config = Config {
        my_cool_var: parsers
            .get(parser_config.get("my_cool_var").unwrap())
            .unwrap()(content.get("my_cool_var").unwrap())
        .as_string()
        .unwrap()
        .clone(),
        my_other_var: parsers
            .get(parser_config.get("my_other_var").unwrap())
            .unwrap()(content.get("my_other_var").unwrap())
        .as_integer()
        .unwrap(),
    };

    Ok(parsed_config)
}
