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
        .zip(tokens.iter().map(|w| morse::to_morse(w)))
        .collect::<HashMap<&String, String>>();

    let reversed_map = map
        .iter()
        .fold(HashMap::<&String, Vec<&String>>::new(), |mut acc, x| {
            acc.entry(x.1).or_insert(Vec::new()).push(x.0);
            acc
        });

    match challenge1(&reversed_map) {
        None => println!("Nothing found for challenge1"),
        Some(morse) => println!(
            "Found {} for challenge1: {:?}",
            morse,
            reversed_map.get(morse).unwrap()
        ),
    };
}

//The sequence -...-....-.--. is the code for four different words (needing, nervate, niding, tiling).
// Find the only sequence that's the code for 13 different words.
fn challenge1<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Option<&'a &'a String> {
    let result = map
        .iter()
        .filter(|e| e.1.len() == 13)
        .map(|e| e.0)
        .collect::<Vec<_>>();

    if result.len() > 1 {
        println!("More than one result? {:?}", result);
    }
    result.get(0).cloned()
}

fn read_tokens(file: &str) -> Result<Vec<String>, io::Error> {
    println!("Opening file");
    let input = fs::read_to_string(file)?;
    let result = input.lines().map(|x| String::from(x.trim())).collect();
    println!("Done reading file");
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
