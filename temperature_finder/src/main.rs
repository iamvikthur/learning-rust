use std::io;

const DAYS_IN_WEEK: usize = 7;

fn main() {
    let mut temperatures = [0.0; DAYS_IN_WEEK]; // Array to store daily temperatures

    for i in 0..DAYS_IN_WEEK {
        println!("Enter temperature for day {}:", i + 1);
        temperatures[i] = get_user_input();
    }

    let average_temperature = calculate_average_temperature(&temperatures);

    println!(
        "Average temperature for the week: {:.2}",
        average_temperature
    );
}

fn get_user_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number:");
                continue;
            }
        }
    }
}

fn calculate_average_temperature(temperatures: &[f64; DAYS_IN_WEEK]) -> f64 {
    let sum: f64 = temperatures.iter().sum();
    sum / DAYS_IN_WEEK as f64
}
