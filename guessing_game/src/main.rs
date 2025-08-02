use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn run_game(game_range: u32) {
    // RANDOM NUMBER
    let secret_number = rand::rng().random_range(1..=game_range);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("❌ Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("❌ Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small!. Try again."),
            Ordering::Greater => println!("📈 Too big!. Try again."),
            Ordering::Equal => {
                println!(
                    "👏🏿 Congratulations! You guessed the number: {}",
                    secret_number
                );
                println!("Do you want to play again? (y/n)");

                let mut play_again = String::new();
                io::stdin()
                    .read_line(&mut play_again)
                    .expect("❌ Failed to read line");

                if play_again.trim().eq_ignore_ascii_case("y") {
                    main();
                }
                break;
            }
        }
    }
}

fn main() {
    println!("🎯 Welcome to the guessing game!");
    println!("Choose game defficulty level between 1 and 3 (1 being easy and 3 being hard)");
    println!("1. Easy (1-10)");
    println!("2. Medium (1-50)");
    println!("3. Hard (1-100)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("❌ Failed to read line");

    let difficulty: u32 = match difficulty.trim().parse() {
        Ok(difficulty) if difficulty >= 1 && difficulty <= 3 => difficulty,
        _ => {
            println!("❌ Invalid difficulty level. Defaulting to Hard (1-100).");
            3
        }
    };

    let game_range: u32 = match difficulty {
        1 => {
            println!("You have chosen easy mode! I am thinking of a number between 1 and 10. Can you guess it?");
            10
        }
        2 => {
            println!("You have chosen Medium mode! I am thinking of a number between 1 and 50. Can you guess it?");
            50
        }
        _ => {
            println!("You have chosen Hard mode! I am thinking of a number between 1 and 100. Can you guess it?");
            100
        }
    };

    run_game(game_range);
}
