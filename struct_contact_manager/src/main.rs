use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    id: u32,
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        println!("\n📇 Contact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contact");
        println!("4. Delete Contact");
        println!("5. Exit");

        let choice = get_input("Please choose an option");
        match choice.trim().parse::<u32>().unwrap_or(0) {
            1 => {
                let contact = create_contact(next_id);
                contacts.push(contact);
                next_id += 1;
                println!("👏🏿 Contact added successfully!");
            }
            2 => view_contacts(&contacts),
            3 => search_contact(&contacts),
            4 => delete_contact(&mut contacts),
            5 => {
                println!("Exiting Contact Manager. Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
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

fn create_contact(id: u32) -> Contact {
    let name = get_input("Enter contact name: ");
    let phone = get_input("Enter contact phone: ");
    let email = get_input("Enter contact email: ");

    Contact {
        id,
        name,
        phone,
        email,
    }
}

fn view_contacts(contacts: &[Contact]) {
    if contacts.is_empty() {
        println!("No contacts available.");
    } else {
        println!("\n📋 Contacts List:");
        for contact in contacts {
            println!(
                "ID: {}, Name: {}, Phone: {}, Email: {}",
                contact.id,
                contact.name.to_string(),
                contact.phone.to_string(),
                contact.email.to_string()
            );
        }
    }
}

fn search_contact(contacts: &[Contact]) {
    let search_name = get_input("Enter name to search: ");
    let found_contacts: Vec<&Contact> = contacts
        .iter()
        .filter(|c| c.name.to_lowercase().contains(&search_name.to_lowercase()))
        .collect();

    if found_contacts.is_empty() {
        println!("No contacts found with the name '{}'.", search_name);
    } else {
        println!("\n🔍 Search Results:");
        for contact in found_contacts {
            println!(
                "ID: {}, Name: {}, Phone: {}, Email: {}",
                contact.id, contact.name, contact.phone, contact.email
            );
        }
    }
}

fn delete_contact(contacts: &mut Vec<Contact>) {
    let id_str = get_input("Enter contact ID to delete: ");
    match id_str.parse::<u32>() {
        Ok(id) => {
            if let Some(pos) = contacts.iter().position(|c| c.id == id) {
                contacts.remove(pos);
                println!("Contact with ID {} has been deleted.", id);
            } else {
                println!("No contact found with ID {}.", id);
            }
        }
        Err(_) => println!("Invalid ID format."),
    }
}
