use std::io::{self, prelude::*, Write};

use crate::{database::Database, employee::{Employee, Role, Salary, WorkType}};


pub fn add_employee(database: &mut Database) {

    let mut name;
    loop {
        name = String::new();
        print!("Input employee name[must be nore than 3 characters]: ");
        io::stdout().flush().unwrap();
        
        if let Err(_) = io::stdin().read_line(&mut name) {
            println!("Failed to read line");
            pause();
            continue;
        }
        name = name.trim().to_string();
        if name.len() <= 3 {
            continue;
        }
        break;
    }


    let mut age: i32;
    loop {
        let mut age_string = String::new();
        print!("Input employee age[>= 17]: ");
        io::stdout().flush().unwrap();
        
        if let Err(_) = io::stdin().read_line(&mut age_string) {
            println!("Failed to read line");
            pause();
            continue;
        }

        age = match age_string.trim().parse()  {
            Ok(age) => {
                age
            },
            Err(_) => {
                println!("Please input a number...");
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
        let mut role_string = String::new();
        print!("Input employee role[Assistant | Programmer](Case Sensitive): ");
        io::stdout().flush().unwrap();


        if let Err(_) = io::stdin().read_line(&mut role_string) {
            println!("Failed to read line");
            pause();
            continue;
        }

        role_string = role_string.trim().to_string();

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
        let mut work_type_string = String::new();
        print!("Input employee type[PartTime | FullTime](Case Sensitive): ");
        io::stdout().flush().unwrap();

        if let Err(_) = io::stdin().read_line(&mut work_type_string) {
            println!("Failed to read line");
            pause();
            continue;
        }

        work_type_string = work_type_string.trim().to_string();

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
        println!("Name: {}", employee.name);
        println!("Age: {}", employee.age);
        println!("Role: {}", employee.role);
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