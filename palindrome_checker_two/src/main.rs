// PALINDOME CHECKET TWO :: STRINGS
use std::io;

fn main() {
    println!("Welcome to the Palindrome Checker!");
    println!("Please enter a string to check if it is a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let cleaned_input = cleaned_input(&input);

    if cleaned_input.is_empty() {
        println!("❌ Please enter a valid non-empty string.");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("✅ The string '{}' is a palindrome.", input);
    } else {
        println!("❌ The string '{}' is not a palindrome.", input);
    }
}

fn cleaned_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
}

fn is_palindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}
