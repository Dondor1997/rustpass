#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("rustpass")
        .version(crate_version!())
        .author(crate_authors!())
        .about("command line password manager")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();
}
