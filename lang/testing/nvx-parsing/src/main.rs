use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::{collections::HashMap, time::Duration};

type Output = HashMap<String, String>;

fn parse_nvx(contents: &str) -> Output {
    let mut output = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split(':');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        output.insert(key.to_string(), value.to_string());
    }

    output
}

fn parse_json(contents: &str) -> Output {
    let mut output = HashMap::new();
    let json: serde_json::Value = serde_json::from_str(contents).unwrap();

    fn parse_json_value(value: &serde_json::Value, output: &mut Output, prefix: &str) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let key = format!("{}{}", prefix, key);
                    parse_json_value(value, output, &key);
                }
            }
            serde_json::Value::String(s) => {
                output.insert(prefix.to_string(), s.to_string());
            }
            _ => {}
        }
    }

    parse_json_value(&json, &mut output, "");

    output
}

fn read_fixture(fixture: &str) -> String {
    let file = format!("fixtures/{}", fixture);
    std::fs::read_to_string(file).unwrap()
}

fn test_nvx(nvx: &str) -> Duration {
    let now = std::time::Instant::now();
    parse_nvx(nvx);

    now.elapsed()
}

fn test_json(json: &str) -> Duration {
    let now = std::time::Instant::now();
    parse_json(json);

    now.elapsed()
}

fn generate_fixtures() {
    let contents = std::fs::read_to_string("fixtures/words.txt").unwrap();
    let words = contents.lines().collect::<Vec<_>>();

    let mut output = HashMap::new();
    for i in 0..words.len() / 2 {
        output.insert(words[i * 2].to_string(), words[i * 2 + 1].to_string());
    }

    let mut nvx = String::new();
    for (key, value) in &output {
        nvx.push_str(&format!("{}:{}\n", key, value));
    }
    std::fs::write("fixtures/test.generated.nvx", nvx).unwrap();

    let json = serde_json::to_string(&output).unwrap();
    std::fs::write("fixtures/test.generated.json", json).unwrap();
}

fn main() {
    env_logger::init();

    if std::env::args().nth(1).unwrap_or_default() == "regen" {
        log::info!("Regenerating fixtures");
        generate_fixtures();
        return;
    }

    let iterations = (0..100).collect::<Vec<_>>();

    log::info!("Starting test - Running {} iterations", iterations.len());

    let nvx = read_fixture("test.generated.nvx");
    let results = iterations
        .par_iter()
        .map(|_| test_nvx(&nvx))
        .collect::<Vec<_>>();
    let a_avg = results.iter().sum::<Duration>() / results.len() as u32;

    print!("nvx done.");
    log::info!("nvx avg: {:?}", a_avg);

    let json = read_fixture("test.generated.json");

    let results = iterations
        .par_iter()
        .map(|_| test_json(&json))
        .collect::<Vec<_>>();
    let b_avg = results.iter().sum::<Duration>() / results.len() as u32;

    print!("json done.");
    log::info!("json avg: {:?}", b_avg);
}
