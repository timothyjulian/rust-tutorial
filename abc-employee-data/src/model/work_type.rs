use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WorkType {
    PartTime,
    FullTime
}

impl WorkType {
    pub fn new(work_type: String) -> Result<WorkType, Box<dyn Error>> {
        if work_type.eq("PartTime") {
            Ok(WorkType::PartTime)
        } else if work_type.eq("FullTime") {
            Ok(WorkType::FullTime)
        } else {
            return Err(Box::new(WorkTypeError::NotFound));
        }
    }
    
}

#[derive(Debug)]
pub enum WorkTypeError{
    NotFound
}

impl Error for WorkTypeError {}

impl fmt::Display for WorkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkTypeError::NotFound => write!(f, "Work type provided not found"),
        }
    }
}