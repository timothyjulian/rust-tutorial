use std::fmt::{self};
use std::{error::Error, io::{self, Write}};

#[derive(Debug)]
pub enum InputError {
    ReadLineError
}

impl Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::ReadLineError => write!(f, "Failed to read line")
        }
    }
}


pub fn get_input_string(message: &str) -> Result<String, Box<dyn Error>> {
    let mut string = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();

    if let Err(_) = io::stdin().read_line(&mut string) {
        return Err(Box::new(InputError::ReadLineError));
    }
    
    string = string.trim().to_string();
    Ok(string)
}

pub fn get_input_i32(message: &str) -> Result<i32, Box<dyn Error>> {
    let mut string = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    if let Err(_) = io::stdin().read_line(&mut string) {
        return Err(Box::new(InputError::ReadLineError));
    }
    
    let num_i32: i32 = string.trim().parse()?;

    Ok(num_i32)
}
