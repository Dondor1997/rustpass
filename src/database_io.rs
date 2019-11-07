use::std::collections::BTreeMap;
use crate::entry::Entry;
use std::fs::{read, write};
use getrandom;

pub fn create_db(path: &str) -> std::io::Result<()> {
    let map: BTreeMap<String, Entry> = BTreeMap::new();
    let secret = gen_secret();

    let database_path = format!("{}{}", path, "rustpass.db");
    let secret_path = format!("{}{}", path, "secret.env");

    write(&secret_path, secret);
    write_db(&database_path, map);
    Ok(())
}

pub fn gen_secret() -> [u8; 32]{
   let mut secret = [0;32];
   getrandom::getrandom(&mut secret);
   secret
}

pub fn read_db(path: &str) -> BTreeMap<String, Entry> {
    let file = read(&path).unwrap();

    let map: BTreeMap<String, Entry> = bincode::deserialize(&file).unwrap();
    map
}

pub fn write_db(path: &str, map: BTreeMap<String, Entry>) -> std::io::Result<()> {
    let encoded = bincode::serialize(&map).unwrap();
    write(&path, encoded)?;
    Ok(())
}
