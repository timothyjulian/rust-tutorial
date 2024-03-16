use std::error::Error;

use crate::model::work_type::WorkType;
use crate::model::role::Role;
use crate::model::salary::Salary;

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub age: i32,
    pub role: Role,
    pub work_type: WorkType,
    pub salary: Salary,
    pub status: bool,
}

impl Employee {
    pub fn new(name: String, age: i32, role: Role, work_type: WorkType, salary: Salary) -> Result<Employee, Box<dyn Error>> {
        Ok(Employee { name, age, role, work_type, salary, status: true })
    }

}
