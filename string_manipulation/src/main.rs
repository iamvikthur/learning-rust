use std::io::{self, Write};

fn main() {
    println!("🛠 String manipulation tool");

    loop {
        println!("\nChoose an operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find Substring");
        println!("6. Replace Text");
        println!("7. Exit");

        let choice = get_input("Enter your choice (1-7): ");
        match choice.trim() {
            "1" => {
                let input = get_input("Enter a string to reverse: ");
                let reversed: String = input.chars().rev().collect();
                println!("Reversed: {}", reversed);
            }
            "2" => {
                let input = get_input("Enter a string to convert to uppercase: ");
                println!("Uppercase: {}", input.to_uppercase());
            }
            "3" => {
                let input = get_input("Enter a string to convert to lowercase: ");
                println!("Lowercase: {}", input.to_lowercase());
            }
            "4" => {
                let input = get_input("Enter a string to trim: ");
                println!("Trimmed: '{}'", input.trim());
            }
            "5" => {
                let input = get_input("Enter the main string: ");
                let substring = get_input("Enter the substring to find: ");
                if input.contains(&substring) {
                    println!("'{}' found in the main string.", substring);
                } else {
                    println!("'{}' not found in the main string.", substring);
                }
            }
            "6" => {
                let input = get_input("Enter the main string: ");
                let from = get_input("Enter the text to replace: ");
                let to = get_input("Enter the replacement text: ");
                let replaced = input.replace(&from, &to);
                println!("Result: {}", replaced);
            }
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
