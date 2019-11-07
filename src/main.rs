#[macro_use]
extern crate clap;
extern crate dirs;
extern crate rpassword;
extern crate crypto;
extern crate getrandom;

use clap::{Arg, App, SubCommand};
use std::path::PathBuf;

mod entry;
mod database_io;

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
        .subcommand(SubCommand::with_name("add")
                    .about("add a new database entry")
                    .arg(Arg::with_name("entry")
                         .takes_value(true)
                         .required(true)
                         .value_name("ENTRY")))
        .subcommand(SubCommand::with_name("del")
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

    if let Some(_) = matches.subcommand_matches("init") {
        database_io::create_db(&path);
    }
    
    let database = database_io::read_db(&path);

    if let Some(command) = matches.subcommand_matches("get") {
        let entry = command.value_of("entry").unwrap();
        println!("{}", database.get(entry).unwrap());
    }

}
