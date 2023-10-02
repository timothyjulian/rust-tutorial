fn main() {
    // if expressions

    let number: i32 = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition: bool = true;

    let number: i32 = if condition { 5 } else { 6 };

    println!("{number}");
}
