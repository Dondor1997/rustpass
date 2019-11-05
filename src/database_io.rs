use::std::collections::BTreeMap;
use crate::entry::Entry;
use std::fs::{read, write};

pub fn create_db(path: &str) -> std::io::Result<()> {
    let map: BTreeMap<String, Entry> = BTreeMap::new();
    write_db(map, path)
}

pub fn read_db(path: &str) -> BTreeMap<String, Entry> {
    let file = read(&path).unwrap();

    let map: BTreeMap<String, Entry> = bincode::deserialize(&file).unwrap();
    map
}

pub fn write_db(map: BTreeMap<String, Entry>, path: &str) -> std::io::Result<()> {
    let encoded = bincode::serialize(&map).unwrap();
    write(&path, encoded)?;
    Ok(())
}
