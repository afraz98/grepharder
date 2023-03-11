use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = Command::new("catharder")
    .author("Anthony Frazier <anthonymfrazier1998@gmail.com>")
    .version("0.0.1")
    .about("cat implemented in Rust.")
    .arg(
      Arg::new("filename")
      .short('f')
      .long("filename")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("File to display.")
      .required(true)
    )
    .get_matches();

    let filename = args.get_one::<String>("filename").expect("required");
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
  
    for (index, line_) in reader.lines().enumerate() {
      let line = line_.unwrap();
      println!("{}: {}", index+1, line);
    }
}
