use std::io::{self, Write};

#[derive(Debug)]
struct Account {
    id: u32,
    balance: f64,
    account_name: String,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. View Account Information");
        println!("4. Deposit");
        println!("5. Withdraw");
        println!("6. Exit");

        match get_input("Choose and option").as_str() {
            "1" => {
                let account = create_account(next_id);
                accounts.push(account);
                next_id += 1;
            }
            "2" => view_balance(&accounts),
            "3" => view_account_information(&accounts),
            "4" => deposit(&mut accounts),
            "5" => withdraw(&mut accounts),
            "6" => {
                println!("Exiting the banking system. Goodbye!");
                break;
            }
            _ => println!("Invalid option, choose another option."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_account(user_id: u32) -> Account {
    let account_name = get_input("Enter account name");
    let initial_balance: f64 = get_input("Enter initial balance").parse().unwrap_or(0.0);

    let new_account = Account {
        id: user_id,
        balance: initial_balance,
        account_name,
    };

    println!("Account created: {:?}", new_account);

    new_account
}

fn view_balance(accounts: &Vec<Account>) {
    let account_id: u32 = get_input("Enter account ID").parse().unwrap_or(0);
    if let Some(account) = accounts.iter().find(|&acc| acc.id == account_id) {
        println!("Account ID: {}, Balance: {}", account.id, account.balance);
    } else {
        println!("Account not found.");
    }
}

fn deposit(accounts: &mut Vec<Account>) {
    let account_id: u32 = get_input("Enter account ID").parse().unwrap_or(0);
    let amount: f64 = get_input("Enter deposit amount").parse().unwrap_or(0.0);

    if let Some(account) = accounts.iter_mut().find(|acc| acc.id == account_id) {
        account.balance += amount;
        println!(
            "Deposited {} to account ID {}. New balance: {}",
            amount, account.id, account.balance
        );
    } else {
        println!("Account not found.");
    }
}

fn withdraw(accounts: &mut Vec<Account>) {
    let account_id: u32 = get_input("Enter account ID").parse().unwrap_or(0);
    let amount: f64 = get_input("Enter withdrawal amount").parse().unwrap_or(0.0);

    if let Some(account) = accounts.iter_mut().find(|acc| acc.id == account_id) {
        if account.balance >= amount {
            account.balance -= amount;
            println!(
                "Withdrew {} from account ID {}. New balance: {}",
                amount, account.id, account.balance
            );
        } else {
            println!("Insufficient funds for withdrawal.");
        }
    } else {
        println!("Account not found.");
    }
}

fn view_account_information(accounts: &Vec<Account>) {
    let account_id: u32 = get_input("Enter account ID").parse().unwrap_or(0);
    if let Some(account) = accounts.iter().find(|&acc| acc.id == account_id) {
        println!(
            "Account ID: {}, Name: {}, Balance: {}",
            account.id, account.account_name, account.balance
        );
    } else {
        println!("Account not found.");
    }
}
