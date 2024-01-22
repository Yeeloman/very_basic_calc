//ttsp
use std::io::{self, Write};
use regex::Regex;
use evalexpr::eval;


fn main() {
    println!("Welcome to a very basic calculator");
    println!("to exit this very basic calculator enter quit");
    loop {
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("failed to read line");
        if input.trim() == "quit" {
            println!("good bye dog");
            break;
        }
        let pattren1 = Regex::new(r".*\D$").unwrap();
        let pattren2 = Regex::new(r"\d+\s+\d+").unwrap();
        if pattren1.is_match(
            &input.trim(),
        ) || pattren2.is_match(
            &input.trim(),
        ) {
            println!("Syntax Error");
            continue;
        }
        let lose_spaces = Regex::new(r"\s+").unwrap();
        let processed_input = lose_spaces.replace_all(input.trim(), "");
        let pattren3 = Regex::new(r"[\+\-\*\/]{2,}").unwrap();
        if pattren3.is_match(&processed_input) {
            println!("Syntax Error");
            continue;
        }
        match eval(&processed_input) {
            Ok(result) => println!("{}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}
