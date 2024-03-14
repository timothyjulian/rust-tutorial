use std::{error::Error, fmt};

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

#[derive(Debug)]
pub enum Salary {
    PartTimeSalary{pay_per_hour: i32, working_hour_per_week: i32},
    FullTimeSalary{base_salary: i32}
}

impl Salary {
    pub fn new_part_time_salary(pay_per_hour: i32, working_hour_per_week: i32) -> Salary {
        Salary::PartTimeSalary { pay_per_hour, working_hour_per_week }
    }
    pub fn new_full_time_salary(base_salary: i32) -> Salary {
        Salary::FullTimeSalary { base_salary }
    }
}


#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub age: i32,
    pub role: Role,
    pub work_type: WorkType,
    pub salary: Salary
}

impl Employee {
    pub fn new(name: String, age: i32, role: Role, work_type: WorkType, salary: Salary) -> Result<Employee, Box<dyn Error>> {
        Ok(Employee { name, age, role, work_type, salary })
    }

}
