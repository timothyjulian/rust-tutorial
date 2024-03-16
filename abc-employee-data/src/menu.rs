use std::io;
use std::io::prelude::*;
use std::io::Write;

use crate::database::Database;
use crate::model::employee::Employee;
use crate::model::role::Role;
use crate::model::work_type::WorkType;
use crate::model::salary::Salary;
use crate::input;

pub fn add_employee(database: &mut Database) {
    let mut name;
    loop {
        name = match input::get_input_string("Input employee name[must be nore than 3 characters]: ")  {
            Ok(name) => name,
            Err(err) => {
                println!("{}", err);
                pause();
                continue;
            }
        };
        if name.len() <= 3 {
            continue;
        }
        break;
    }


    let mut age: i32;
    loop {
        age = match input::get_input_i32("Input employee age[>=17]: ")  {
            Ok(age) => {
                age
            },
            Err(err) => {
                println!("{}", err);
                pause();
                continue;
            }
        };

        if age < 17 {
            continue;
        }

        break;
    }


    let role: Role;

    loop {
        let role_string = match input::get_input_string("Input employee role[Assistant | Programmer](Case Sensitive): ") {
            Ok(role_string) => role_string,
            Err(err) => {
                println!("{}", err);
                pause();
                continue;
            }
        };

        if role_string.ne("Assistant") && role_string.ne("Programmer") {
            continue;
        }

        role = match Role::new(role_string) {
            Ok(role) => role,
            Err(err) => {
                println!("Error creating role: {}", err);
                pause();
                continue;
            }
        };

        break;
    }

    let work_type: WorkType;
    loop {
        let work_type_string = match input::get_input_string("Input employee type[PartTime | FullTime](Case Sensitive): ") {
            Ok(work_type_string) => work_type_string,
            Err(err) => {
                println!("{}", err);
                pause();
                continue;
            }
        };

        if work_type_string.ne("PartTime") && work_type_string.ne("FullTime") {
            continue;
        }

        work_type = match WorkType::new(work_type_string) {
            Ok(work_type) => work_type,
            Err(err) => {
                println!("Error creating work type: {}", err);
                pause();
                continue;
            }
        };

        break;
    }

    let salary = salary_input(&work_type);

    match Employee::new(name, age, role, work_type, salary) {
        Ok(new_employee) => {
            println!("Success insert employee data\n");
            pause();
            database.add_employee(new_employee);
        }
        Err(err) => {
            println!("Error creating employee: {}", err);
        }
    }
}

pub fn get_all_employee(database: &Database) {
    let employee_list = database.get_all_employee();
    if employee_list.len() <= 0 {
        println!("There is no employee data in the list\n");
        return;
    }
    for i in 0..employee_list.len() {
        println!("\n\nEmployee no.{}", i+1);
        let employee = employee_list.get(i).unwrap();
        match employee.work_type{
            WorkType::FullTime => {
                println!("Full Time Employee");
            }
            WorkType::PartTime => {
                println!("Part Time Employee");
            }
        }
        println!("========================");
        match employee.status {
            true => {
                println!("Status: active");
            }
            false => {
                println!("Status: not active");
            }
        }
        println!("Name: {}", employee.name);
        println!("Age: {}", employee.age);
        println!("Role: {}", employee.role);
        match employee.salary {
            Salary::FullTimeSalary { base_salary } => {
                println!("Base salary per month: {}", base_salary);
            }
            Salary::PartTimeSalary { pay_per_hour, working_hour_per_week } => {
                println!("Pay per hour: {}", pay_per_hour);
                println!("Working hour per week: {}", working_hour_per_week);
                println!("Salary per month: {}", pay_per_hour * working_hour_per_week);
            }
            
        }
    }
    println!("\n")
}

pub fn resign_employee(database: &mut Database) {
    get_all_employee(&database);
    let employee_list: &mut Vec<Employee> = database.get_mutable_all_employee();

    let mut resign_index: i32;
    loop {
        print!("Input employee number that wat to resign[1..{}]: ", employee_list.len());
        io::stdout().flush().unwrap();
        let mut resign_index_string = String::new();
        if let Err(_) = io::stdin().read_line(&mut resign_index_string) {
            println!("Failed to read line");
            pause();
            continue;
        }
        
        resign_index = match resign_index_string.trim().parse() {
            Ok(resign_index) => resign_index,
            Err(_) => {
                println!("Failed to parse resign index");
                pause();
                continue;
            }
        };

        if resign_index < 1 || resign_index > employee_list.len() as i32 {
            continue;
        }
        break;
    }
    resign_index -=1;
    let employee = employee_list.get_mut(resign_index as usize).unwrap();
    match employee.status {
        true =>  {
            println!("{} is resigning...", employee.name);
            employee.status = false;
            pause();
            return;
        }
        false => {
            println!("Employee has already resigned!");
            return;
        }
        
    }
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn salary_input(work_type: &WorkType) -> Salary {
    match work_type {
        WorkType::PartTime => {
            let mut pay_per_hour: i32;
            loop {
                let mut pay_per_hour_string = String::new();
                print!("Input pay per hour[>=10000]: ");
                io::stdout().flush().unwrap();
                
                if let Err(_) = io::stdin().read_line(&mut pay_per_hour_string) {
                    println!("Failed to read line");
                    pause();
                    continue;
                }

                pay_per_hour = match pay_per_hour_string.trim().parse() {
                    Ok(pay_per_hour) => {
                        pay_per_hour
                    }
                    Err(_) => {
                        println!("Failed to parse pay per hour");
                        pause();
                        continue;
                    }
                };

                if pay_per_hour < 10000 {
                    continue;
                }

                break;
            }


            let mut working_hour_per_week: i32;
            loop {
                let mut working_hour_per_week_string = String::new();
                print!("Input working hour per week[>0]: ");
                io::stdout().flush().unwrap();

                if let Err(_) = io::stdin().read_line(&mut working_hour_per_week_string)  {
                    println!("Failed to read line");
                    pause();
                    continue;
                }

                working_hour_per_week = match working_hour_per_week_string.trim().parse() {
                    Ok(working_hour_per_week) => {
                        working_hour_per_week
                    }
                    Err(_) => {
                        println!("Failed to parse working hour per week");
                        pause();
                        continue;
                    }
                };

                if working_hour_per_week <= 0 {
                    continue;
                }

                break;
            }

            Salary::new_part_time_salary(pay_per_hour, working_hour_per_week)
        },
        WorkType::FullTime => {
            let mut base_salary: i32;
            loop {
                let mut base_salary_string = String::new();
                print!("Input base salary[>=10000]: ");
                io::stdout().flush().unwrap();

                if let Err(_) = io::stdin().read_line(&mut base_salary_string) {
                    println!("Failed to read line");
                    pause();
                    continue;
                }

                base_salary = match base_salary_string.trim().parse() {
                    Ok(base_salary) => base_salary,
                    Err(_) => {
                        println!("Failed to parse base salary");
                        pause();
                        continue;
                    }
                    
                };


                if base_salary < 10000 {
                    continue;
                }

                break;
            }
            Salary::new_full_time_salary(base_salary)
        }
    }
}