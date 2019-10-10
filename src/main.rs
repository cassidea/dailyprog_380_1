#[macro_use]
extern crate lazy_static;
mod morse;

use std::collections::HashMap;
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

    let map = tokens
        .iter()
        .cloned()
        .zip(tokens.iter().map(|w| morse::to_morse(w)))
        .collect::<HashMap<String, String>>();

    let reversed_map = map
        .iter()
        .fold(HashMap::<String, Vec<String>>::new(), |mut acc, x| {
            acc.entry(x.1.clone())
                .or_insert(Vec::new())
                .push(x.0.clone());
            acc
        });

    let morsed = map.get("aah").unwrap();
    println!("{}", morsed);
    let unmorsed = reversed_map.get(morsed).unwrap();

    println!("Reversed result {} : {:?}", morsed, unmorsed);
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
