fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break can return a value;
        }
    };
    println!("{result}");

    let mut count = 0; 
    'counting_up: loop { // labeled loop 
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // inner loop breaking
            }

            if count == 2 {
                break 'counting_up; // will break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count: {count}");

    let mut number = 3;

    println!("while");
    while number != 0 { // while
        println!("{number}");

        number -= 1;
    }

    let arrays: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("for in arrays");
    for number_loop  in arrays {
        println!("{number_loop}");
    }

    println!("for in range");
    for number_loop in 1..=20 {
        println!("{number_loop}");
    }

    println!("for in range reverse");
    for number_loop in (1..4).rev() {
        println!("{number_loop}");
    }



}
