use std::collections::HashMap;

//The sequence -...-....-.--. is the code for four different words (needing, nervate, niding, tiling).
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

//autotomous encodes to .-..--------------..-..., which has 14 dashes in a row.
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
