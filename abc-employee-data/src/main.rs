use std::error;
use std::io::{self, prelude::*, Write};
use std::process;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    loop {
        let mut menu: String = String::new();

        println!("ABC EMPLOYEE DATA");
        println!("==================");
        println!("1. Add employee");
        println!("2. Vie employee");
        println!("3. Resign");
        println!("4. Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        if let Err(_) = io::stdin().read_line(&mut menu) {
            println!("Failed to read line");
            pause();
        }

        let menu: u32 = match menu.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please input a number...");
                pause();
                continue;
            }
        };

        if menu < 1 || menu > 4 {
            println!("Please input a valid menu...");
            pause();
            continue;
        }


    }
}
