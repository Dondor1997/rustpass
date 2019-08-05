#[macro_use]
extern crate clap;
extern crate dirs;
extern crate rpassword;

use clap::{Arg, App};
use std::path::PathBuf;
use std::collections::HashMap;
use entry::Entry;
use std::fs::{read, write};

mod entry;

fn create_db(path: &str) -> HashMap<String, Entry> {
    let map: HashMap<String, Entry> = HashMap::new();
    map
}

fn read_db(path: &str) -> HashMap<String, Entry> {
   let file = read(&path).unwrap();

    let hashmap: HashMap<String, Entry> = bincode::deserialize(&file).unwrap();
    hashmap
}

fn write_db(hashmap: HashMap<String, Entry>, path: &str) -> std::io::Result<()> {
    let encoded = bincode::serialize(&hashmap).unwrap();
    write(&path, encoded)?;
    Ok(())
}

fn main() {
    let matches = App::new("rustpass")
        .version(crate_version!())
        .author(crate_authors!())
        .about("command line password manager")
        .arg(Arg::with_name("path")
             .short("p")
             .long("path")
             .help("sets alternative database path")
             .takes_value(true)
             .value_name("path"))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("mode")
             .help("select behaviour")
             .required(true)
             .index(1)
             .possible_values(&["insert", "delete", "get", "init", "search", "list"]))
        .arg(Arg::with_name("key")
             .help("key to be modified")
             .required(true)
             .index(2))
        .get_matches();

    let mut default: PathBuf  = dirs::home_dir().unwrap();
    default.push(".rustpass.db");

    let path = matches
        .value_of("path")
        .unwrap_or(default.to_str().unwrap());

    let mode = matches.value_of("mode").unwrap();
    let key = matches.value_of("key").unwrap();

    let mut db: HashMap<String, Entry> = match mode {
        "init" => create_db(path),
        _ => read_db(path),
    };
    
    let passphrase = rpassword::read_password_from_tty(Some("Please enter passphrase: ")).unwrap();

    if mode == "insert" {
        let entry = Entry::new(key.to_string(), passphrase);
        db.insert(key.to_string(), entry); 
    } else if mode == "get" {
        let entry = db.get(key).expect("No database entry under this key");
        println!("{}", entry);
    } else if mode == "delete" {
        
    } else if mode == "search" {

    } else if mode == "list" {
        
    }

    write_db(db, path).unwrap();
}
