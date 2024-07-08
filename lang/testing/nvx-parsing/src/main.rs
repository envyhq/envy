use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

type Output = HashMap<String, String>;

// parses the following into hashmap
//
// name:wow
// again:cool
// why:not
//
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
    if std::env::args().nth(1).unwrap_or_default() == "regen" {
        generate_fixtures();
        return;
    }

    let test_cases = (0..1000).collect::<Vec<_>>();

    let a_avg = Arc::new(Mutex::new(None));
    let a_avg_inner = a_avg.clone();
    let test_cases_a = test_cases.clone();

    let b_avg = Arc::new(Mutex::new(None));
    let b_avg_inner = Arc::downgrade(&b_avg);
    let test_cases_b = test_cases.clone();

    let a = thread::spawn(move || {
        let nvx = read_fixture("test.generated.nvx");
        let results = test_cases_a
            .par_iter()
            .map(|_| test_nvx(&nvx))
            .collect::<Vec<_>>();
        a_avg_inner.lock().unwrap().replace(Some(
            results.iter().sum::<Duration>() / results.len() as u32,
        ));
    });

    let b = thread::spawn(move || {
        let json = read_fixture("test.generated.json");
        let results = test_cases_b
            .par_iter()
            .map(|_| test_json(&json))
            .collect::<Vec<_>>();
        b_avg_inner.upgrade().unwrap().lock().unwrap().replace(Some(
            results.iter().sum::<Duration>() / results.len() as u32,
        ));
    });

    let start = std::time::Instant::now();

    let a_avg_tick_inner = a_avg.clone();
    let b_avg_tick_inner = b_avg.clone();
    let c = thread::spawn(move || {
        while a_avg_tick_inner.lock().unwrap().is_none()
            || b_avg_tick_inner.lock().unwrap().is_none()
        {
            print!("\x1B[2J\x1B[1;1H");
            log::info!("Elapsed: {:?}", start.elapsed());
        }
    });

    a.join().unwrap();
    b.join().unwrap();
    c.join().unwrap();

    log::info!("nvx avg: {:?}", a_avg.lock().unwrap().unwrap().unwrap());
    log::info!("json avg: {:?}", b_avg.lock().unwrap().unwrap().unwrap());
}
