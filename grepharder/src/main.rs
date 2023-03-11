use regex::Regex;
use clap::{arg, Command};
 
fn main() {
    let args = Command::new("grepharder")
                .author("Anthony Frazier <anthonymfrazier1998@gmail.com>")
                .version("0.0.0")
                .about("grep implemented in Rust.")
                .arg (arg!(--pattern <VALUE>).required(true))
                .get_matches();

    let regex = Regex::new(args.get_one::<String>("pattern").expect("required")).unwrap();
    let text = "\
Hi how are ya. I was wondering if I could get a picture with you.
You're a big celebrity and a picture with you would mean the world to me.
What do you think?";
    
    for (index, line) in text.lines().enumerate() {
        let contains = regex.find(line);
        match contains {
            Some(_) => println!("{}: {}", index+1, line),
            None => (),
        }
    }
}
