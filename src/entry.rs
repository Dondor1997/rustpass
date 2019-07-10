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
}
