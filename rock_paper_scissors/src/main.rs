use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("🎮 Welcome to Rock, Paper, Scissors!");
    print!("Instructions: Enter 'rock', 'paper', or 'scissors'. Type 'exit' to quit: ");

    loop {
        print!("\n🪨 📰 ✂ Make your choice \n");

        let user_choice = get_user_choice();
        if user_choice == "exit" {
            println!("\n 👋 Thanks for playing! Goodbye!");
            break;
        }

        let computer_choice = get_computer_choice();
        println!("\n🤖 Computer chose: {}", computer_choice);

        let result = determine_winner(&user_choice, &computer_choice);

        let game_output: &str = match result {
            GameResult::UserWins => "🎉 You win!",
            GameResult::ComputerWins => "😢 You lose!",
            GameResult::Draw => "🤝 It's a draw!",
        };

        println!("\n{}", game_output);

        // io::stdout().flush().unwrap();
        let play_again = get_play_again_choice();
        if play_again != "yes" {
            println!("\n👋 Thanks for playing! Goodbye!");
            break;
        }
    }
}

fn get_user_choice() -> String {
    let choice = get_terminal_input();
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "rock" | "paper" | "scissors" | "exit" => choice,
        _ => {
            println!("\n❗ Invalid choice. Please enter 'rock', 'paper', or 'scissors'.");
            get_user_choice()
        }
    }
}

fn get_terminal_input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_index = rand::rng().random_range(0..choices.len());
    choices[random_index].to_string()
}

fn get_play_again_choice() -> String {
    print!("\n🔄 Do you want to play again? (yes/no): ");
    let choice = get_terminal_input();
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "yes" | "no" => choice,
        _ => {
            println!("\n❗ Invalid choice. Please enter 'yes' or 'no'.");
            get_play_again_choice()
        }
    }
}

enum GameResult {
    UserWins,
    ComputerWins,
    Draw,
}

fn determine_winner(user: &str, computer: &str) -> GameResult {
    if user == computer {
        GameResult::Draw
    } else if (user == "rock" && computer == "scissors")
        || (user == "paper" && computer == "rock")
        || (user == "scissors" && computer == "paper")
    {
        GameResult::UserWins
    } else {
        GameResult::ComputerWins
    }
}
