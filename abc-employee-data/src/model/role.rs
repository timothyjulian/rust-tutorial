use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Role {
    Assistant,
    Programmer
}

impl Role {
    pub fn new(role: String) -> Result<Role, Box<dyn Error>> {
        if role.eq("Assistant") {
            Ok(Role::Assistant)
        } else if role.eq("Programmer") {
            Ok(Role::Programmer)
        } else {
            return Err(Box::new(RoleError::NotFound));
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Assistant => {
                write!(f, "Assistant")
            }
            Role::Programmer => {
                write!(f, "Programmer")
            }
        }
    }
}

#[derive(Debug)]
pub enum RoleError{
    NotFound
}

impl Error for RoleError {}

impl fmt::Display for RoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleError::NotFound => write!(f, "Role provided not found"),
        }
    }
}