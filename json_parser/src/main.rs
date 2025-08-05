use serde_json::Value;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => {
                println!(
                    "✅Parsed JSON:\n{}",
                    serde_json::to_string_pretty(&json).unwrap()
                );
            }
            Err(e) => {
                eprintln!("❌Error parsing JSON: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}
