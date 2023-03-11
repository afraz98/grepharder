use clap::{arg, Command};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = Command::new("catharder")
    .author("Anthony Frazier <anthonymfrazier1998@gmail.com>")
    .version("0.0.0")
    .about("cat implemented in Rust.")
    .arg (arg!(--file <VALUE>).required(true))
    .get_matches();

    let filename = args.get_one::<String>("file").expect("required");

}
