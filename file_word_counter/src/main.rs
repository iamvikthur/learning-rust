use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("❌ Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("📄 Reading file: {}", file_path);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("❌ Error reading file: {}", e);
        return;
    }

    let word_count = contents.split_whitespace().count();
    let char_count = contents.chars().count();
    let line_count = contents.lines().count();

    println!("📊 Word count: {}", word_count);
    println!("📊 Character count: {}", char_count);
    println!("📊 Line count: {}", line_count);
}
