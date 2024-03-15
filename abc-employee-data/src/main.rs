mod database;
mod employee;
mod menu;

use std::error;
use std::io::{self, Write};
use std::process;

use database::Database;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut database = Database::new();
    loop {
        let mut menu: String = String::new();

        println!("\nABC EMPLOYEE DATA");
        println!("==================");
        println!("1. Add employee");
        println!("2. View employee");
        println!("3. Resign");
        println!("4. Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        if let Err(_) = io::stdin().read_line(&mut menu) {
            println!("Failed to read line");
            menu::pause();
        }

        let menu: u32 = match menu.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please input a number...");
                menu::pause();
                continue;
            }
        };

        match menu {
            1 => {
                menu::add_employee(&mut database);
            }
            2 => {
                menu::get_all_employee(&database);
                menu::pause();
            }
            3 => {
                menu::resign_employee(&mut database)
            }
            4 => {
                process::exit(1);
            }
            _ => {
                println!("Please input a valid menu...");
                menu::pause();
                continue;
            }
        };
    }
}
