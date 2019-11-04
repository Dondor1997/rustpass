#[macro_use]
extern crate clap;
extern crate dirs;
extern crate rpassword;

use clap::{Arg, App, SubCommand};
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
             .value_name("PATH"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .help("sets the config file")
             .takes_value(true)
             .value_name("CONFIG"))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("insert")
                    .about("insert new database entry")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY")))
        .subcommand(SubCommand::with_name("delete")
                    .about("delete a database entry")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY")))
        .subcommand(SubCommand::with_name("get")
                    .about("get an entry from the database")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY"))
                    .arg(Arg::with_name("output")
                         .short("o")
                         .help("prints entry to stdout instead to clipboard")))
        .subcommand(SubCommand::with_name("search")
                    .about("search the database by entry name")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY")))
        .subcommand(SubCommand::with_name("list")
                    .about("print all database entrys")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY")))
        .subcommand(SubCommand::with_name("init")
                    .about("initialise an empty database"))
        .get_matches();

    let mut default: PathBuf  = dirs::home_dir().unwrap();
    default.push(".local/share/rustpass/");

    let path = matches
        .value_of("path")
        .unwrap_or(default.to_str().unwrap());
}
