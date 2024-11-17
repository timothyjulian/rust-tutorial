use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut strs_clone = strs.clone();
    for str in &mut strs_clone {
        let mut chars: Vec<char> = str.chars().collect();
        chars.sort();
        *str = chars.into_iter().collect();
    }
    // println!("{:?}", strs_clone);

    let mut hash_map :HashMap<String, Vec<String>>= HashMap::new();
    let mut i = 0;
    for str in &strs_clone {
        let index_vec = hash_map.entry(str.to_owned()).or_insert(vec![]);
        index_vec.push(strs.get(i as usize).unwrap().to_owned());
        
        i += 1;
    }

    // println!("{:?}", hash_map);

    hash_map.into_iter().map(|(_, v)| v).collect()

}

#[test]
fn test_group_anagrams() {
    println!("{:?}", group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]));
    println!("{:?}", group_anagrams(vec!["".to_string()]));
    println!("{:?}", group_anagrams(vec!["a".to_string()]));
}


fn main() {
    println!("Hello, world!");
}
