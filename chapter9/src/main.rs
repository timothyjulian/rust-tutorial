use std::fs::File;

fn main() {
    println!("Hello, world!");

    // this will cause the program to be exit
    // panic!("crash and burn"); 

    // you can recover error using Result enum
    let greeting_file_result = File::open("test.txt");
    // TODO
}
