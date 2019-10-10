use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<char, &'static str> = vec![
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
    ]
    .iter()
    .copied()
    .collect();
}

pub fn to_morse(org_string: &str) -> String {
    org_string
        .chars()
        .map(|c| to_single_morse(c))
        .collect::<Vec<&str>>()
        .join("")
}

fn to_single_morse<'a>(c: char) -> &'a str {
    MAP.get(&c).unwrap()
}

#[test]
fn single_morse_test() {
    assert_eq!(to_single_morse('a'), ".-");
}

#[test]
fn to_morse_test() {
    assert_eq!(to_morse("aaa"), ".-.-.-");
}
