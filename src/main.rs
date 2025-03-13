use menu_rs::{Menu, MenuOption};
use rpassword::read_password;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

struct PasswordEntry {
    service: String,
    username: String,
    password: String,
}

fn get_secure_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    read_password().unwrap()
}

fn add_password() {
    let service = dialoguer::Input::<String>::new()
        .with_prompt("Service")
        .interact()
        .unwrap();

    let username = dialoguer::Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .unwrap();

    let password = get_secure_input("Password: ");

    let entry = PasswordEntry {
        service,
        username,
        password,
    };

    save_to_file(&entry).unwrap();
}

const STORAGE_FILE: &str = "vault.dat";

fn save_to_file(entry: &PasswordEntry) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(STORAGE_FILE)?;

    writeln!(
        file,
        "{} | {} | {}",
        entry.service, entry.username, entry.password
    )
}

fn list_passwords() {
    let file = File::open(STORAGE_FILE).unwrap();
    let reader = BufReader::new(file);

    println!("\nSaved Passwords:");
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" | ").collect();    
        println!("{}. Service: {}\n Username: {}\n Password: ******\n", i + 1, parts[0], parts[1]);
    }
}

fn main() -> std::io::Result<()> {
    loop {
        let main_menu = Menu::new(vec![
            MenuOption::new("Add Password", add_password),
            MenuOption::new("List Passwords", list_passwords),
            MenuOption::new("Exit", || std::process::exit(0)),
        ]);
        
        main_menu.show();
    }
}