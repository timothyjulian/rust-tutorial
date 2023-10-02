use std::io;

fn main() {
    loop {
        println!("\n===Temperature converter==\n");
        println!("1. Celcius to Fahrenheit");
        println!("2. Fahrenheit to Celcius");
        println!("0. Exit");

        let mut menu: String = String::new();

        io::stdin()
            .read_line(&mut menu)
            .expect("Error reading line");

        let menu: u8 = match menu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        match menu {
            1 => celcius_to_fahrenheit(),
            2 => fahrenheit_to_celcius(),
            0 => {
                println!("Program exitted");
                break;
            }
            _ => println!("Unknown command"),
        }
    }
}

fn celcius_to_fahrenheit() {
    println!("Welcome to Celcius to Fahrenheit converter");
    println!("Please input the value in Celcius");

    let mut celcius: String = String::new();

    io::stdin()
        .read_line(&mut celcius)
        .expect("Error reading line");

    let celcius: f64 = match celcius.trim().parse() {
        Ok(celcius) => celcius,
        Err(_) => {
            println!("Please input a number");
            return;
        }
    };

    let fahrenheit: f64 = celcius / 5f64 * 9f64 + 32f64;
    println!("The temperature in fahrenheit is {fahrenheit}");
}

fn fahrenheit_to_celcius() {
    println!("Welcome to Fahrenheit to Celcius converter");
    println!("Please input the value in Fahrenheit");

    let mut fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Error reading line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(fahrenheit) => fahrenheit,
        Err(_) => {
            println!("Please input a number");
            return;
        }
    };

    let celcius: f64 = (fahrenheit - 32f64) / 9f64 * 5f64;
    println!("The temperature in celcius is {celcius}");
}
