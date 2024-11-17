use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }


    let mut stored_char = HashMap::new();
    for c in s.chars() {
        let count = stored_char.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t.chars() {
        if let Some(occurance) = stored_char.get_mut(&c) {
            *occurance -= 1;
            if *occurance < 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_anagram() {
    assert_eq!(
        is_anagram("anagram".try_into().unwrap(), "nagaram".try_into().unwrap()),
        true
    );
    assert_eq!(
        is_anagram("rat".try_into().unwrap(), "car".try_into().unwrap()),
        false
    );
    assert_eq!(
        is_anagram("aacc".try_into().unwrap(), "ccac".try_into().unwrap()),
        false
    );
}

fn main() {
    println!("Hello, world!");
}
