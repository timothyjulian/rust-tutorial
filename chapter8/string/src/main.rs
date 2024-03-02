fn main() {
    // string
    let mut s = String::new();

    let s_literal = "Testing".to_string(); // "Testing" is string literal/ slices

    let s_initial = String::from("Testing");

    let mut s_mutable = String::from("Hi");

    s_mutable.push_str(" there!");
    
    println!("{}", s_mutable);

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1.push_str(&s2);
    

    println!("{}", s1);
    println!("{}", s2);

    let s3 = format!("{}-{}", s1, s2);
    println!("{}", s3);

    // String can not use indexing, must use this way
    let x = s1.chars().nth(1).unwrap_or_default();
    println!("{}", x);
    


}
