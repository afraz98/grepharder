use regex::Regex;
use clap::{Arg, ArgAction, Command};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = Command::new("grepharder")
                .author("Anthony Frazier <anthonymfrazier1998@gmail.com>")
                .version("0.0.1")
                .about("grep implemented in Rust.")
                .arg(
                    Arg::new("pattern")
                    .short('p')
                    .long("pattern")
                    .action(ArgAction::Set)
                    .value_name("PATTERN")
                    .help("Pattern to search provided file argument for (case-sensitive).")
                    .required(true)
                )
                .arg(
                    Arg::new("filename")
                    .short('f')
                    .long("filename")
                    .action(ArgAction::Set)
                    .value_name("FILE")
                    .help("Filename to search for pattern.")
                    .required(true)
                )
                .get_matches();

    let regex = Regex::new(args.get_one::<String>("pattern").expect("required")).unwrap();
    let filename = args.get_one::<String>("filename").expect("required");
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    
    for (index, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        let contains = regex.find(&line);
        match contains {
            Some(_) => println!("{}: {}", index+1, line),
            None => (),
        }
    }

}
