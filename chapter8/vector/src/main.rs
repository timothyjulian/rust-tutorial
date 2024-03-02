

use std::net::TcpListener;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<u32> = Vec::new();
    // no need to define data type
    let v_macro = vec![1, 2, 3, 4, 5];
    let mut mutable_v = Vec::new();
    mutable_v.push(5);
    mutable_v.push(6);
    dbg!(&mutable_v);

    let third: &i32 = &v_macro[2];
    dbg!(third);
    let third_get = v_macro.get(2);
    dbg!(third_get);
    dbg!(v_macro.get(5));

    match third_get {
        Some(third) => {
            println!("The third element is {}", third);
        },
        None => println!("There is no third element."),
    };

    // will make the program panic
    // let first = &v_macro[0];
    // v_macro.push(6);
    // println!("The first element is: {}", first);

    // iterate over vector
    for i in &v_macro {
        println!("{}", i);
    }



    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //iterate and mutate
    for i in &mut mutable_v {
        *i += 50;
    }
    dbg!(&mutable_v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(row);

    



}
