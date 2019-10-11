use std::collections::HashMap;

// The sequence -...-....-.--. is the code for four different words (needing, nervate, niding, tiling).
// Find the only sequence that's the code for 13 different words.
pub fn challenge1<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Option<&'a &'a String> {
    let result = find_by_length(map, 13);
    if result.len() > 1 {
        println!("More than one result? {:?}", result);
    }
    result.get(0).cloned()
}

fn find_by_length<'a>(
    map: &'a HashMap<&String, Vec<&String>>,
    length: usize,
) -> Vec<&'a &'a String> {
    map.iter()
        .filter(|e| e.1.len() == length)
        .map(|e| e.0)
        .collect::<Vec<_>>()
}

// autotomous encodes to .-..--------------..-..., which has 14 dashes in a row.
// Find the only word that has 15 dashes in a row.
pub fn challenge2<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Option<&'a &'a String> {
    find_dashes_in_a_row(map, 15)
}

fn find_dashes_in_a_row<'a>(
    map: &'a HashMap<&String, Vec<&String>>,
    limit: usize,
) -> Option<&'a &'a String> {
    for k in map.keys() {
        if k.len() < limit {
            continue;
        };
        let mut counter = 0;
        for (i, c) in k.chars().enumerate() {
            if c == '.' {
                if k.len() - i <= limit {
                    break;
                } else {
                    counter = 0;
                    continue;
                }
            }
            counter += 1;
        }
        if counter == limit {
            return Some(k);
        }
    }
    None
}

// Call a word perfectly balanced if its code has the same number of dots as dashes.
// counterdemonstrations is one of two 21-letter words that's perfectly balanced. Find the other one.
pub fn challenge3<'a>(map: &'a HashMap<&String, String>) -> Option<(&'a String, &'a String)> {
    let result = find_balanced_words(map, 21);
    match result.len() {
        2 => {
            if *(result.get(0).unwrap().0) == String::from("counterdemonstrations") {
                result.get(1).cloned()
            } else {
                result.get(0).cloned()
            }
        }
        length => {
            println!("Got {} results! Returning nothing! ", length);
            None
        }
    }
}

fn find_balanced_words<'a>(
    map: &'a HashMap<&String, String>,
    limit: usize,
) -> Vec<(&'a String, &'a String)> {
    let mut result = Vec::<(&String, &String)>::new();
    for (w, m) in map {
        if w.len() != limit {
            continue;
        }

        let mut dashes = 0;
        let mut dots = 0;
        for c in m.chars() {
            match c {
                '.' => dots += 1,
                '-' => dashes += 1,
                _ => panic!("Unknown char {}", c),
            };
        }

        if dots == dashes {
            result.push((&w, &m));
        }
    }
    result
}

// protectorate is 12 letters long and encodes to .--..-.----.-.-.----.-..--., which is a
// palindrome (i.e. the string is the same when reversed). Find the only 13-letter word that
// encodes to a palindrome.
pub fn challenge4<'a>(map: &'a HashMap<&String, String>) -> Option<(&'a String, &'a String)> {
    let result = find_palindrome(map, 13);
    match result.len() {
        1 => Some(*result.get(0).unwrap()),
        l => {
            println!("Found {} results! Returning None!", l);
            None
        }
    }
}

fn find_palindrome<'a>(
    map: &'a HashMap<&String, String>,
    limit: usize,
) -> Vec<(&'a String, &'a String)> {
    let mut result = Vec::new();

    'words: for (w, m) in map {
        if w.len() != limit {
            continue;
        }
        for (i, r) in m.chars().zip(m.chars().rev()) {
            if i != r {
                continue 'words;
            }
        }
        result.push((*w, m));
    }
    result
}

#[test]
fn find_by_length_test() {
    let mut map = HashMap::new();
    let morse1 = String::from("..---");
    let morse2 = String::from("...");
    map.insert(&morse1, Vec::<&String>::new());

    let w1 = &String::from("a");
    let w2 = &String::from("b");
    let w3 = &String::from("c");
    map.insert(&morse2, vec![w1, w2, w3]);

    let result = find_by_length(&map, 3);

    match result.len() {
        0 => panic!("Nothing found!"),
        1 => assert_eq!(&morse2, **result.get(0).unwrap()),
        _ => panic!("More than one result"),
    };
}

#[test]
fn find_dashes_in_a_row_test() {
    let mut map = HashMap::new();
    let dashes = String::from("..---");
    let dots = String::from("--...");
    map.insert(&dashes, Vec::<&String>::new());
    map.insert(&dots, Vec::<&String>::new());
    match find_dashes_in_a_row(&map, 3) {
        None => panic!("Did not find '---'"),
        Some(morse) => assert_eq!(dashes, **morse),
    };
}

#[test]
fn find_balanced_words_test() {
    let mut map = HashMap::new();
    let w1 = String::from("a");
    let m1 = String::from(".-");
    let w2 = String::from("aa");
    let m2 = String::from(".-.-");
    let w3 = String::from("ee");
    let m3 = String::from("..");
    map.insert(&w1, m1);
    map.insert(&w2, m2);
    map.insert(&w3, m3);

    let result = find_balanced_words(&map, 2);
    match result.len() {
        1 => assert_eq!(w2, **result.get(0).unwrap().0),
        l => panic!("Wrong number {} of results!", l),
    }
}

#[test]
fn find_palindrome_test() {
    let mut map = HashMap::new();
    let w1 = String::from("a");
    let m1 = String::from(".-");
    let w2 = String::from("aa");
    let m2 = String::from(".-.-");
    let w3 = String::from("eee");
    let m3 = String::from("...");
    map.insert(&w1, m1);
    map.insert(&w2, m2);
    map.insert(&w3, m3);

    let result = find_palindrome(&map, 3);
    match result.len() {
        1 => assert_eq!(w3, *result.get(0).unwrap().0),
        l => panic!("Wrong number {} of results!", l),
    }
}
