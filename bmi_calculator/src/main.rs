// SIMPLE BMI CALCULATOR
use std::io;

fn main() {
    println!("Welcome to the BMI calculator");

    // WEIGHT
    println!("Please enter your weight in kg:");
    let weight = match get_input_as_f64() {
        Some(val) => val,
        None => {
            println!("Invalid input for weight. Please enter a valid number.");
            return;
        }
    };

    // HEIGHT
    println!("Please enter your height in meters:");
    let height = match get_input_as_f64() {
        Some(val) => val,
        None => {
            println!("Invalid input for height. Please enter a valid number.");
            return;
        }
    };

    if height <= 0.0 || weight <= 0.0 {
        println!("Weight and height must be positive numbers.");
        return;
    }

    // BMI CALCULATION
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);

    let category = categorize_bmi(bmi);
    println!("You are classified as: {}", category);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<f64>().ok()
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn categorize_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 24.9 {
        "Normal weight"
    } else if bmi < 29.9 {
        "Overweight"
    } else {
        "Obesity"
    }
}
