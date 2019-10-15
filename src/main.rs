#[macro_use]
extern crate lazy_static;
mod challenges;
mod morse;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;
use std::time;

const INPUT_FILE: &str = "enable1.txt";

fn main() {
    if !path::Path::new(INPUT_FILE).exists() {
        panic!("Needed input does not exist {}!", INPUT_FILE)
    }
    let tokens = read_tokens(INPUT_FILE).unwrap();
    println!("Number of tokens {}", tokens.len());

    let start = time::Instant::now();

    let map = tokens
        .iter()
        .fold(HashMap::<&String, String>::new(), |mut acc, w| {
            acc.insert(w, morse::to_morse(w));
            acc
        });

    let reversed_map = map
        .iter()
        .fold(HashMap::<&String, Vec<&String>>::new(), |mut acc, x| {
            acc.entry(x.1).or_insert_with(Vec::new).push(x.0);
            acc
        });

    let start_challenges = time::Instant::now();
    println!(
        "Created maps in {:?}",
        start_challenges.duration_since(start)
    );

    let start_challenge1 = time::Instant::now();
    match challenges::challenge1(&reversed_map) {
        None => println!("Nothing found for challenge 1"),
        Some(morse) => println!(
            "Found {} -> {:?} for challenge 1 in {:?}",
            morse,
            reversed_map.get(morse).unwrap(),
            start_challenge1.elapsed()
        ),
    };

    let start_challenge2 = time::Instant::now();
    match challenges::challenge2(&reversed_map) {
        None => println!("Nothing found for challenge 2"),
        Some(morse) => println!(
            "Found {} -> {:?} for challenge 2 in {:?}",
            morse,
            reversed_map.get(morse).unwrap(),
            start_challenge2.elapsed()
        ),
    };

    let start_challenge3 = time::Instant::now();
    match challenges::challenge3(&map) {
        None => println!("Nothing found for challenge 3"),
        Some((word, morse)) => println!(
            "Found {} -> {} for challenge 3 in {:?}",
            word,
            morse,
            start_challenge3.elapsed()
        ),
    };

    let start_challenge4 = time::Instant::now();
    match challenges::challenge4(&map) {
        None => println!("Nothing found for challenge 4"),
        Some((word, morse)) => println!(
            "Found {} -> {} for challenge 4 in {:?}",
            word,
            morse,
            start_challenge4.elapsed()
        ),
    };

    for (func_name, func) in vec![
        (
            "challenge5_contains",
            challenges::challenge5_contains as fn(&HashMap<&String, Vec<&String>>) -> Vec<String>,
        ),
        (
            "challenge5_contains_startswith",
            challenges::challenge5_contains_startswith,
        ),
        (
            "challenge5_contains_by_hand",
            challenges::challenge5_contains_by_hand,
        ),
        (
            "challenge5_contains_java",
            challenges::challenge5_contains_java,
        ),
        (
            "challenge5_contains_memcmp",
            challenges::challenge5_contains_memcmp,
        ),
    ] {
        let start_challenge5 = time::Instant::now();
        let c5 = func(&reversed_map);
        if c5.len() == 4 {
            println!("Missing sequences for challenge 5 are: {:?}", c5);
        } else {
            println!("Found {} missing sequences e.g. {:?}", c5.len(), c5);
            for k in reversed_map.keys() {
                if k.contains("-.---.------.") {
                    println!("{} has not been found!", k);
                }
            }
        }
        println!(
            "Challenge 5 {} took {:?}",
            func_name,
            start_challenge5.elapsed()
        );
    }

    println!(
        "Finished all challenges in {:?}",
        start_challenges.elapsed()
    );
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
