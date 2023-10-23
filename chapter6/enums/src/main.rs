#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // can also use struct as the parameter
    V6(String),
}

enum Message { // this also possible
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // something here with all the enums
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
    
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));


    dbg!(&home);
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 2, y: 3 };

    quit_message.call();
    move_message.call();

    let a = value_in_cents(Coin::Quarter(UsState::Alabama));
    dbg!(Coin::Quarter(UsState::Alaska));

    dbg!(a);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } 
    // else { // here some code }


}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn route(ip_kind: IpAddrKind) {

}

