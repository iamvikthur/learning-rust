use std::io;

fn main() {
    println!("Welcome to the Simple Calculator!");
    println!("-----------------------------");
    println!("Available operations are: +, -, *, /");
    println!("Please enter your operation in the format: number1 operator number2");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let vectors: Vec<&str> = input.trim().split_whitespace().collect();

    if vectors.len() != 3 {
        println!("Invalide input format. Please use: number1 operator number2");
        return;
    }

    let number1: f64 = match vectors[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number: {}", vectors[0]);
            return;
        }
    };

    let operator = vectors[1];

    let number2: f64 = match vectors[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number: {}", vectors[2]);
            return;
        }
    };

    let result = match operator {
        "+" => add(number1, number2),
        "-" => subtract(number1, number2),
        "*" => multiply(number1, number2),
        "/" => divide(number1, number2),
        _ => {
            println!("Invalid operator: {}", operator);
            return;
        }
    };

    println!("Result: {:.2}", result);
    println!("Thank you for using the Simple Calculator!");
    println!("Have a great day!");
    println!("If you want to perform another calculation, please restart the program.");
    println!("Goodbye!");
    println!("----------------------------");
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Division by zero is not allowed.");
        return 0.0; // or handle it as you prefer
    }
    a / b
}
