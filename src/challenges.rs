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

// --.---.---.-- is one of five 13-character sequences that does not appear in the encoding
// of any word. Find the other four.
pub fn challenge5<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse2(map, 13)
}

#[allow(dead_code)]
fn find_missing_morse1<'a>(map: &'a HashMap<&String, Vec<&String>>, limit: u32) -> Vec<String> {
    let perms = get_permutations(limit, vec!["--.---.---.--".to_string()]);
    println!("Got {} permutations", perms.len());
    let mut result = perms.clone();

    let keys = map
        .keys()
        .filter(|k| k.len() >= limit as usize)
        .collect::<Vec<_>>();
    println!("Found {} keys to check", keys.len());
    for (i, p) in perms.iter().enumerate().rev() {
        for k in keys.iter() {
            if k.contains(p) {
                result.remove(i);
                break;
            }
        }
    }
    result
}

fn find_missing_morse2<'a>(map: &'a HashMap<&String, Vec<&String>>, limit: u32) -> Vec<String> {
    let mut perms = get_permutations(limit, vec!["--.---.---.--".to_string()]);

    perms.retain(|p| {
        for k in map.keys() {
            if contains(k, p) {
                return false;
            }
        }
        true
    });
    perms
}

#[allow(dead_code)]
fn find_missing_morse3<'a>(map: &'a HashMap<&String, Vec<&String>>, limit: u32) -> Vec<String> {
    let perms = get_permutations(limit, vec!["--.---.---.--".to_string()]);
    let mut result = Vec::<String>::new();

    for p in perms.iter() {
        let mut found = false;
        for k in map.keys() {
            if k.contains(p) {
                found = true;
                break;
            }
        }
        if !found {
            result.push(p.clone());
        }
    }
    result
}

fn get_permutations(limit: u32, to_be_ignored: Vec<String>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for i in 0..2_u32.pow(limit) {
        let s_i = format!("{:width$b}", i, width = limit as usize);
        let morse = s_i
            .chars()
            .map(|c| match c {
                '0' | ' ' => '.',
                '1' => '-',
                x => panic!("Unknown char {}!", x),
            })
            .collect::<String>();
        if !to_be_ignored.contains(&morse) {
            result.push(morse);
        }
    }
    result
}

fn contains(haystack: &str, needle: &str) -> bool {
    #[cfg(test)]
    println!("Searching {} in {}", needle, haystack);

    if needle.len() == 0 {
        return true;
    }

    if haystack.len() == 0 {
        return false;
    }

    let mut h_i = 0;
    let mut h;
    'haystack: loop {
        match haystack.get(h_i..h_i + 1) {
            Some(x) => (h = x),
            None => return false,
        };

        #[cfg(test)]
        println!("haystack beginning: h: {}, h_i {}", h, h_i);
        let mut n_i = 0;
        'needle: loop {
            #[cfg(test)]
            println!("needle beginning: h: {}, h_i {}", h, h_i);
            let remaining_haystack = haystack.len() - h_i;
            let remaining_needle = needle.len() - n_i - 1;

            if remaining_needle > remaining_haystack {
                #[cfg(test)]
                println!(
                    "Needle len {} is longer than remaining haystack {} ({}, {}) -> false",
                    remaining_needle,
                    remaining_haystack,
                    haystack.len(),
                    h_i
                );
                return false;
            }

            let n = needle.get(n_i..n_i + 1).unwrap();
            if n != h {
                #[cfg(test)]
                println!("Continue haystack n: {}, h:{}", n, h);
                h_i = h_i - n_i + 1; //h_i - n_i = current letter, +1 => next letter
                continue 'haystack;
            }
            #[cfg(test)]
            println!(
                "n==h: Current n: {}, n_peek is: {:?}",
                n,
                needle.get(n_i + 1..n_i + 2)
            );
            if remaining_needle == 0 {
                #[cfg(test)]
                println!("found needle {} in haystack {}!", needle, haystack);
                #[cfg(test)]
                println!("***************************************");
                return true;
            }

            match haystack.get(h_i + 1..h_i + 2) {
                Some(temp_h) => {
                    #[cfg(test)]
                    println!("Continuing needle");
                    h_i += 1;
                    h = temp_h;
                    n_i += 1;
                }
                None => {
                    if needle.get(n_i + 1..n_i + 2).is_some() {
                        return false;
                    }
                }
            }
        }
    }
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

#[test]
fn get_permutations_test() {
    let limit: u32 = 3;
    let total: u32 = 2_u32.pow(limit);

    let test_perms = ["...", "-..", ".-.", "..-", "--.", "-.-", ".--", "---"];
    assert_eq!(total as usize, test_perms.len());

    let perms = get_permutations(limit, Vec::<String>::new());
    for p in test_perms.iter() {
        assert!(
            perms.contains(&p.to_string()),
            "'{}' is missing in {:?}",
            p,
            perms
        );
    }
}

#[test]
fn get_permutations_test2() {
    let limit: u32 = 13;
    let total: u32 = 2_u32.pow(limit);

    let perms = get_permutations(limit, Vec::<String>::new());
    assert_eq!(total as usize, perms.len());
}

#[test]
fn find_missing_morse_test() {
    let test_perms = [
        String::from("-..--"),
        String::from(".-.--"),
        String::from("..--."),
        String::from("--.--"),
        String::from("-.-.."),
        String::from(".--.."),
    ];
    let mut map = HashMap::new();
    for p in test_perms.iter() {
        map.insert(p, Vec::<&String>::new());
    }

    let missing_perms = find_missing_morse2(&map, 3);
    assert!(
        missing_perms.contains(&String::from("...")),
        "'...' is found?"
    );
    assert!(
        missing_perms.contains(&String::from("---")),
        "'---' is found?"
    );
    assert_eq!(
        2,
        missing_perms.len(),
        "More elements than expected {:?}",
        missing_perms
    );
}

#[test]
fn contains_test() {
    vec![
        ("a", "a"),
        ("test", "te"),
        ("test", "es"),
        ("test", "st"),
        ("test", "test"),
        ("", ""),
        ("", "a"),
        ("a", ""),
        ("a", "b"),
        ("test", "abc"),
        ("test", "testtest"),
        ("abcabcd", "abcd"),
        ("-.-.---.------..-...-.--.", "-.---.------."),
    ]
    .iter()
    .for_each(|(haystack, needle)| {
        assert_eq!(
            haystack.contains(needle),
            contains(haystack, needle),
            "Wrong result for {} in {}",
            needle,
            haystack
        );
    });
}
