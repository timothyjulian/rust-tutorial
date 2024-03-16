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
