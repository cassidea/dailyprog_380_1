#[macro_use]
extern crate lazy_static;
mod morse;

use std::fs;
use std::io;
use std::path;

const INPUT_FILE: &str = "enable1.txt";

fn main() {
    if !path::Path::new(INPUT_FILE).exists() {
        panic!("Needed input does not exist {}!", INPUT_FILE)
    }
    let tokens = read_tokens(INPUT_FILE).unwrap();
    println!("Number of tokens {}", tokens.len());

    println!(
        "3.element '{}' in morse '{}'",
        tokens.get(3).unwrap(),
        morse::to_morse(tokens.get(3).unwrap())
    )
}

fn read_tokens(file: &str) -> Result<Vec<String>, io::Error> {
    println!("Opening file");
    let input = fs::read_to_string(file)?;
    let result = input.lines().map(|x| String::from(x.trim())).collect();
    Ok(result)
}

#[test]
fn read_tokens_test() {
    assert_eq!(
        path::Path::new(INPUT_FILE).exists(),
        true,
        "Expected input file does not exist! {}",
        INPUT_FILE
    );
    assert_eq!(
        read_tokens(INPUT_FILE).unwrap().len(),
        172823,
        "Number of tokens differ!"
    );
}

#[test]
fn to_morse_main_test() {
    assert_eq!(morse::to_morse("abc"), ".--...-.-.");
}
