use std::io;

fn main() {
    println!("Welcome to the Prime Number Checker!");
    println!("Enter a positive integer to check if it's a prime numeber:");

    let number = match get_input_as_u32() {
        Some(num) => num,
        None => {
            println!("Invalid input. Please enter a valid positive integer.");
            return;
        }
    };

    if number <= 0 {
        println!("Please enter a positive integer greater than 0.");
        return;
    }

    let primes = calculate_prime(number);

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is NOT a prime number.", number);
    }

    println!("The prime numbers up to {} are: {:?}", number, primes);
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<u32>().ok()
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn calculate_prime(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}
