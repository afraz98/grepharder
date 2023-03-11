use regex::Regex;
use clap::{arg, Command};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = Command::new("grepharder")
                .author("Anthony Frazier <anthonymfrazier1998@gmail.com>")
                .version("0.0.0")
                .about("grep implemented in Rust.")
                .arg (arg!(--pattern <VALUE>).required(true))
                .arg (arg!(--file <VALUE>).required(true))
                .get_matches();

    let regex = Regex::new(args.get_one::<String>("pattern").expect("required")).unwrap();
    let filename = args.get_one::<String>("file").expect("required");
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
