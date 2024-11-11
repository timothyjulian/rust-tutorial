use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn day_of_programmer(year: i32) -> String {
    if year <= 1917 && year >= 1700 {
        let is_leap_year: bool = is_julian_leap_year(year);
        return get_day_of_month(256, is_leap_year, year);
    } else if year == 1918 {
        return calculate_transition();
    } else {
        let is_leap_year: bool = is_gregorian_leap_year(year);
        return get_day_of_month(256, is_leap_year, year);
    }
}

fn is_julian_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        return true;
    }
    false
}

fn calculate_transition() -> String {
    // since this is constant, we can just use constant
    String::from("26.09.1918")
}

fn is_gregorian_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    } else if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    false
}

#[test]
fn test_day_of_programmer() {
    println!("{}", day_of_programmer(2019)); 
}

fn get_day_of_month(mut day: i32, is_leap: bool, year: i32) -> String {
    let days_in_month: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut current_month = 0;
    while day >= days_in_month[current_month] + if is_leap && current_month == 1 { 1 } else { 0 } {
        day -= days_in_month[current_month] + if is_leap && current_month == 1 { 1 } else { 0 };
        current_month += 1;
    }
    // println!("{}", current_month + 1);
    // println!("{}", day);

    // if it end up as 0
    day = if day == 0 {
        days_in_month[current_month]
    } else {
        day
    };

    // format for dd
    let day = if day < 10 {
        format!("0{}", day)
    } else {
        format!("{}", day)
    };

    // format for mm
    let month = if current_month < 10 {
        format!("0{}", current_month + 1)
    } else {
        format!("{}", current_month)
    };

    format!("{}.{}.{}", day, month, year)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
