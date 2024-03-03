use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let key = "a";
    let value = map.get(key).copied().unwrap_or(0);

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // insert if non existent
    map.entry(key).or_insert(5);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", "test");

}
