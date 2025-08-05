use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path> [--lines] [--search <search_term>]");
        return;
    }

    let file_path = &args[1];
    let show_lines = args.contains(&"--lines".to_string());
    // let search_term_index = args.iter().position(|x| x == "--search");
    let search_term = if let Some(index) = args.iter().position(|x| x == "--search") {
        args.get(index + 1).cloned()
    } else {
        None
    };

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", file_path, e);
            return;
        }
    };

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                if let Some(ref term) = search_term {
                    if content.contains(term) {
                        if show_lines {
                            println!("{}: {}", index + 1, content);
                        } else {
                            println!("{}", content);
                        }
                    }
                } else {
                    if show_lines {
                        println!("{}: {}", index + 1, content);
                    } else {
                        println!("{}", content);
                    }
                }
            }
            Err(e) => eprintln!("Error reading line {}: {}", index + 1, e),
        }
    }
}
