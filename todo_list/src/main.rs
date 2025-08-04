use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    println!("Welcome to the Todo List Application!");

    let mut todo_items: Vec<TodoItem> = load_todo_items();

    loop {
        println!("\n📔 Todo Tasks Menu:");
        println!("1. Add Todo Item");
        println!("2. View Todo Items");
        println!("3. Mark Todo Item as Completed");
        println!("4. Delete Todo Item");
        println!("5. Save and Exit");

        let choice = get_user_input("Choose an option (1-5): ");

        match choice.trim() {
            "1" => add_todo_item(&mut todo_items),
            "2" => view_todo_items(&todo_items),
            "3" => mark_todo_item_completed(&mut todo_items),
            "4" => delete_todo_item(&mut todo_items),
            "5" => {
                save_todo_items(&todo_items);
                println!("Todo items saved. Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_todo_items() -> Vec<TodoItem> {
    let file_path = "todo_items.json";
    if let Ok(file_content) = fs::read_to_string(file_path) {
        serde_json::from_str(&file_content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_todo_items(todo_items: &Vec<TodoItem>) {
    let file_path = "todo_items.json";
    let json_data = serde_json::to_string_pretty(todo_items).expect("Failed to serialize tasks");
    let mut file = File::create(file_path).expect("Failed to save tasks");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
}

fn add_todo_item(todo_items: &mut Vec<TodoItem>) {
    let description = get_user_input("Enter todo item description: ");
    let last_todo_id = todo_items.last().map_or(0, |item| item.id);
    let id = last_todo_id as u32 + 1;
    let new_item = TodoItem {
        id,
        description,
        completed: false,
    };
    todo_items.push(new_item);
    println!("Todo item added.");
}

fn view_todo_items(todo_items: &Vec<TodoItem>) {
    if todo_items.is_empty() {
        println!("No todo items found.");
    } else {
        for item in todo_items {
            println!(
                "{}. [{}] {}",
                item.id,
                if item.completed { "✅" } else { "❌" },
                item.description
            );
        }
    }
}

fn mark_todo_item_completed(todo_items: &mut Vec<TodoItem>) {
    let id_str = get_user_input("Enter the ID of the todo item to mark as completed: ");
    if let Ok(id) = id_str.parse::<u32>() {
        if let Some(item) = todo_items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Todo item marked as completed.");
        } else {
            println!("Todo item with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID format.");
    }
}

fn delete_todo_item(todo_items: &mut Vec<TodoItem>) {
    let id_str = get_user_input("Enter the ID of the todo item to delete: ");
    if let Ok(id) = id_str.parse::<u32>() {
        if let Some(pos) = todo_items.iter().position(|i| i.id == id) {
            todo_items.remove(pos);
            println!("Todo item deleted.");
        } else {
            println!("Todo item with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID format.");
    }
}
