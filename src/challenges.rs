use libc;
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
            if &result.get(0).unwrap().0[..] == "counterdemonstrations" {
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
pub fn challenge5_contains<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse(map, 13, self::contains_in_rust)
}

pub fn challenge5_contains_startswith<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse(map, 13, self::contains_starts_with)
}

pub fn challenge5_contains_by_hand<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse(map, 13, self::contains_by_hand)
}

pub fn challenge5_contains_java<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse(map, 13, self::contains_java)
}

pub fn challenge5_contains_memcmp<'a>(map: &'a HashMap<&String, Vec<&String>>) -> Vec<String> {
    find_missing_morse(map, 13, self::contains_memcmp)
}

#[allow(dead_code)]
fn find_missing_morse<'a>(
    map: &'a HashMap<&String, Vec<&String>>,
    limit: u32,
    func: fn(&str, &str) -> bool,
) -> Vec<String> {
    let mut perms = get_permutations(limit, vec!["--.---.---.--".to_string()]);
    perms.retain(|p| {
        for k in map.keys() {
            if func(k, p) {
                return false;
            }
        }
        true
    });
    perms
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

#[allow(dead_code)]
fn contains_by_hand(haystack_str: &str, needle_str: &str) -> bool {
    #[cfg(test)]
    println!("Searching {} in {}", needle_str, haystack_str);

    if needle_str.is_empty() {
        return true;
    }

    if haystack_str.is_empty() {
        return false;
    }

    let haystack = haystack_str.as_bytes();
    let needle = needle_str.as_bytes();

    #[cfg(test)]
    println!("haystack_bytes {:?} needle_bytes {:?}", haystack, needle);

    let mut h_i = 0;
    let mut h;
    'haystack: loop {
        match get(haystack, h_i) {
            Some(x) => (h = x),
            None => return false,
        };

        #[cfg(test)]
        println!("haystack beginning: h: {}, h_i {}", h, h_i);
        let mut n_i = 0;
        /*'needle:*/
        loop {
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

            let n = get(needle, n_i).unwrap();
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
                get(needle, n_i + 1)
            );
            if remaining_needle == 0 {
                #[cfg(test)]
                println!("found needle {} in haystack {}!", needle_str, haystack_str);
                #[cfg(test)]
                println!("***************************************");
                return true;
            }

            match get(haystack, h_i + 1) {
                Some(temp_h) => {
                    #[cfg(test)]
                    println!("Continuing needle");
                    h_i += 1;
                    h = temp_h;
                    n_i += 1;
                }
                None => {
                    if get(needle, n_i + 1).is_some() {
                        return false;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn get(haystack: &[u8], i: usize) -> Option<&u8> {
    haystack.get(i)
}

fn contains_starts_with(haystack_str: &str, needle_str: &str) -> bool {
    #[cfg(test)]
    println!("Searching {} in {}", needle_str, haystack_str);

    if needle_str.is_empty() {
        return true;
    }

    if haystack_str.is_empty() || haystack_str.len() < needle_str.len() {
        return false;
    }

    let haystack = haystack_str.as_bytes();
    let needle = needle_str.as_bytes();

    for i in 0..=haystack.len() - needle.len() {
        #[cfg(test)]
        println!("{:?}.starts_with({:?})", &haystack[1..], needle);
        let current_haystack = &haystack[i..];
        if current_haystack.len() < needle.len() {
            return false;
        }
        if current_haystack.starts_with(needle) {
            return true;
        }
    }
    false
}

fn contains_memcmp(haystack_str: &str, needle_str: &str) -> bool {
    #[cfg(test)]
    println!("Searching {} in {}", needle_str, haystack_str);

    if needle_str.is_empty() {
        return true;
    }

    if haystack_str.is_empty() || haystack_str.len() < needle_str.len() {
        return false;
    }

    let haystack = haystack_str.as_bytes();
    let needle = needle_str.as_bytes();

    for i in 0..=haystack.len() - needle.len() {
        #[cfg(test)]
        println!("{:?}.starts_with({:?})", &haystack[1..], needle);
        let current_haystack = &haystack[i..i + needle.len()];
        if current_haystack.len() < needle.len() {
            return false;
        }
        if memcmp_eq(current_haystack, needle) {
            return true;
        }
    }
    false
}

fn memcmp_eq<'a, T: PartialEq>(a: &'a [T], b: &'a [T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    unsafe { libc::memcmp(a.as_ptr() as *const _, b.as_ptr() as *const _, a.len()) == 0 }
}

fn contains_java(haystack: &str, needle: &str) -> bool {
    #[cfg(test)]
    println!("Searching {} in {}", needle, haystack);
    return index_of(
        haystack.as_bytes(),
        0,
        haystack.len() as isize,
        needle.as_bytes(),
        0,
        needle.len(),
        0,
    ) > -1;
}

fn index_of(
    source: &[u8],
    source_offset: isize,
    source_count: isize,
    target: &[u8],
    target_offset: usize,
    target_count: usize,
    from_index: isize,
) -> isize {
    if from_index >= source_count {
        match target_count {
            0 => return source_count,
            _ => return -1,
        };
    }

    if target_count == 0 {
        return from_index;
    }

    let first = target[target_offset];
    #[cfg(test)]
    println!("{} + {} - {}", source_offset, source_count, target_count);
    let max = source_offset + (source_count - target_count as isize);

    for mut i in source_offset + from_index..=max {
        let mut ui = i as usize;
        /* Look for first character. */
        if source[ui] != first {
            ui += 1;
            while ui <= max as usize && source[ui] != first {
                ui += 1;
            }
        }

        i = ui as isize;
        if i <= max {
            /* Found first character, now look at the rest of v2 */
            let mut j = ui + 1;
            let end = j + target_count - 1;
            let mut k = target_offset + 1;
            while j < end && source[j] == target[k] {
                j += 1;
                k += 1;
            }

            if j == end {
                /* Found whole string. */
                return i - source_offset;
            }
        }
    }

    return -1;
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

    for t in "test".as_bytes() {}

    for (func_name, func) in vec![
        (
            "contains_in_rust",
            self::contains_in_rust as fn(haystack: &str, needle: &str) -> bool,
        ),
        ("contains_starts_with", self::contains_starts_with),
        ("contains_by_hand", self::contains_by_hand),
        ("contains_java", self::contains_java),
        ("contains_memcmp", self::contains_memcmp),
    ] {
        #[cfg(test)]
        println!("Testing {}", func_name);
        let missing_perms = find_missing_morse(&map, 3, func);
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
}

#[test]
fn contains_in_rust_test() {
    println!("Testing contains_in_rust()");
    contains_abstract_test(self::contains_in_rust);
}

fn contains_in_rust(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

#[test]
fn contains_starts_with_test() {
    println!("Testing contains_starts_with()");
    contains_abstract_test(self::contains_starts_with);
}

#[test]
fn contains_by_hand_test() {
    println!("Testing contains_by_hand()");
    contains_abstract_test(self::contains_by_hand);
}

#[test]
fn contains_java_test() {
    println!("Testing contains_in_java()");
    contains_abstract_test(self::contains_java);
}

#[test]
fn contains_memcmp_test() {
    println!("Testing contains_in_java()");
    contains_abstract_test(self::contains_memcmp);
}

#[cfg(test)]
fn contains_abstract_test(func: fn(&str, &str) -> bool) {
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
            func(haystack, needle),
            "Wrong result for {} in {}",
            needle,
            haystack
        );
    });
}
