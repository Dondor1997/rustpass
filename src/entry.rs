use std::{fmt, io};
use serde::{Serialize, Deserialize};
use rpassword;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Entry {
    title: String,
    username: String,
    password: String,
    notes: String,
}

impl Entry {
    
    pub fn new(title: String, passphrase: String) -> Entry {
        let username = read_from_stdin("Username: ");
        let password = rpassword::read_password_from_tty(Some("Password:")).unwrap();
        println!("");
        let notes = read_from_stdin("Additional Notes: ");
        Entry { title, username, password, notes }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_notes(&self) -> &str {
        &self.notes
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " Key: {}\r\n Username: {}\r\n Password: {}\r\n Notes: {}", self.title, self.username, self.password, self.notes)
    }
}

fn read_from_stdin(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("");
    buffer.trim().to_string()
}
