use std::io::{self, Write};
use std::{thread, time::Duration};

fn main() {
    println!("⌛ Basic Timer ⌛");

    let duration = match get_duration_input() {
        Some(d) => d,
        None => {
            println!("Invalid input. Please enter a valid duration in the format 'HH:MM:SS'.");
            return;
        }
    };

    println!(
        "Time set for {} hours, {} minutes, and {} seconds.",
        duration.0, duration.1, duration.2
    );

    start_timer(duration.0, duration.1, duration.2);

    println!("\nTime is up!");
}

fn get_duration_input() -> Option<(u64, u64, u64)> {
    print!("Enter duration (HH:MM:SS): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let parts: Vec<&str> = input.trim().split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64) {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    for remaining in (0..total_seconds).rev() {
        let hrs = remaining / 3600;
        let mins = (remaining % 3600) / 60;
        let secs = remaining % 60;

        print!("\rTime remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
