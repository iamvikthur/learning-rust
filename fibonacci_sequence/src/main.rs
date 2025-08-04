use std::io;

fn main() {
    println!("WELCOME TO THE FIBONACCI SEQUENCE GENERATOR!");
    println!("Enter the number of terms you want in the Fibonacci sequence:");

    let number_of_terms = get_input_as_u32("Number of terms (positive integer):");

    if number_of_terms == 0 {
        println!("The Fibonacci sequence is empty for 0 terms");
        return;
    }

    let sequence = calculate_fibonacci(number_of_terms);

    println!(
        "The first {} terms of the Fibonacci sequence are:",
        number_of_terms
    );
    println!("{:?}", &sequence);
}

fn get_input_as_u32(prompt: &str) -> u32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            get_input_as_u32(prompt)
        }
    }
}

fn calculate_fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = vec![0, 1];
    for i in 2..n as usize {
        let next_term = sequence[i - 1] + sequence[i - 2];
        sequence.push(next_term);
    }
    sequence.truncate(n as usize);
    sequence
}
