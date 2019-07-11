#[macro_use]
extern crate clap;
extern crate dirs;

use clap::{Arg, App};
use std::path::PathBuf;

mod entry;

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
    default.push("rustpass.db");

    let path = matches
        .value_of("path")
        .unwrap_or(default.to_str().unwrap());

}
