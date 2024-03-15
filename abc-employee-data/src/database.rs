use crate::employee::Employee;

pub struct Database {
    pub employee_list: Vec<Employee>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            employee_list: vec![],
        }
    }

    pub fn add_employee(&mut self, employee: Employee) {
        self.employee_list.push(employee);
    }

    pub fn get_all_employee<'a>(&'a self) -> &'a Vec<Employee> {
        &self.employee_list
    }

    pub fn get_mutable_all_employee<'a>(&'a mut self) -> &'a mut Vec<Employee> {
        &mut self.employee_list
    }
}
