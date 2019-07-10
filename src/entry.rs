use std::fmt;

pub struct Entry {
    title: String,
    username: String,
    password: String,
    notes: String,
}

impl Entry {
    
    pub fn new(title: String, username: String, password: String, notes: String, passphrase: String) -> Entry {

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
        write!(f, "{}\r\n Username: {}\r\n Password: {}\r\n Notes: {}", self.title, self.username, self.password, self.notes)
    }
}
