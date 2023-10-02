use std::io;

fn main() {
    // mutability (because variables are immutable by default)
    let mut x: i32 = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("constant: {THREE_HOURS_IN_SECONDS}");

    // shadowing -> redeclaration of y
    let y: i32 = 9;
    let y: i32 = y + 1;
    {
        let y: String = y.to_string();
        println!("y converted to string: {y} and its length: {}", y.len());
    }
    println!(
        "y still an i32: {y} and if it is multiplied by 5: {}",
        y * 5
    );

    // data types
    // rust is statically typed languge, therefore for example when using
    // .parse, need to annotate the data type
    // let static_var: u32 = "32".parse().expect("Can't parse"); // if we don't add u32 rust will complain

    // ================
    // scalar types -> represents a single value
    // 4 primary scalar types -> int, floating point, char, boolean

    // integer -> i8, i6, i32 .., i28 or i can be replaced with u
    // i -> signed integer means can hold negative number
    // u -> unsigned integer means can not hold negative number

    let thousand: i32 = 1_000; // _ can be used to be visual separator
    let hexa: i32 = 0xff;
    let octal: i32 = 0o77;
    let binary: i32 = 0b1111_0000;

    println!("{thousand}");
    println!("{hexa}");
    println!("{:x}", hexa);
    println!("{octal}");
    println!("{:o}", octal);
    println!("{binary}");
    println!("{:b}", binary);

    // floating
    let floating_x: f64 = 9.0; // by default is f64
    let floating_y: f32 = 9.0;

    println!("{}", floating_x);
    println!("{}", floating_y);

    // example of arithmetic
    println!("{}", 56 as f64 / 32 as f64);
    println!("{}", 56.2 / 34.2);

    // boolean
    let t: bool = true;
    println!("{}", t);

    // char
    let sushi: char = '🍣';
    println!("{}", sushi);

    // =====================
    // compound types -> group multiple value into one type.
    // primitive -> arrays & tuples

    // tuples -> can be different types
    let tup: (&str, i32, f64, char) = ("Test", 2_000, 5.4, '🍣'); // fixed size

    let (a, b, c, d) = tup;

    println!("The value of b is {b}");
    println!("num 2: {}", tup.1);

    // arrays -> must be same data type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let same: [i32; 5] = [0; 5]; // [0, 0, 0, 0, 0]
    println!("{}", a[3]);
    println!("{}", same[0]);
    output_array_index(a);

    another_function();
    print_measurement("Timo", 12);

    // statement -> does not return a value
    let statement: i32 = 6;
    // let statement = statement2 = 6 is not allowed in Rust

    // expression -> return a value
    let expression: i32 = {
        let x = 5;
        x + 2 // this return a value therefore an expression -> notice the missing semicolon
    };

    // function with return statement
    let return_value: i32 = function_with_return(expression);
}

fn function_with_return(parameter: i32) -> i32 {
    parameter * 5 // same as return parameter * 5
}

fn output_array_index(a: [i32; 5]) {
    println!("Array: {:?}", a);
    println!("Input number index: ");

    let mut index: String = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Error reading line");

    let index: u64 = index
        .trim()
        .parse()
        .expect("Can not convert to integer for index.");

    println!("Data for index {} is {}", index, a[index as usize]);
}

// function can be defined anywhere
fn another_function() {
    println!("Test from another function");
}

fn print_measurement(word: &str, length: i32) {
    println!("Test {word} + {length}");
}
