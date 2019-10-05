use std::fs;
use std::io;

fn main() {
    let input_file = "enable1.txt";
    let tokens = read_tokens(input_file).unwrap();
    println!("Number of tokens {}", tokens.len());
}

fn read_tokens(input_file: &str) -> Result<Vec<String>, io::Error> {
    println!("Opening file");
    let input = fs::read_to_string(input_file)?;
    let result = input.lines().map(|x| String::from(x.trim())).collect();
    Ok(result)
}

#[test]
fn hello_world() {
    println!("Hello Gitlab CI");
    assert!(false)
}
