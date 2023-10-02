use std::io;

fn main() {
    println!("Fibonacci number sequence generator");
    println!("Input the fibonacci index");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Error reading line");

    let index: i32 = index.trim().parse().expect("Not a parseable number");

    for i in  0..=index {
        print!("{} ", get_fibonacci(i));
    }

}

fn get_fibonacci(index: i32) -> i32 {

    let (mut a, mut b) = (0, 1);
    if index == 0 {
        return a;
    } else if index == 1 {
        return b;
    }
    let mut c: i32 = 0;
    for _i in 2..=index {
        c = a + b;
        a = b;
        b = c; 
    }
    c
}