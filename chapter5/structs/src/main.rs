struct User {
    // struct
    activate: bool, // field
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32);

struct AlwaysEqual; // unit-like struct with no data

fn main() {
    let mut user1 = User {
        username: String::from("test"),
        activate: true,
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };
    user1.sign_in_count += 1; // to change the value of struct need to make the instance mutable

    println!("{}", user1.sign_in_count);

    let user2 = build_user(String::from("test"), String::from("test"));

    let user3 = User {
        email: String::from("Test"),
        ..user2 // user2 moved here
    };

    println!("{}", user2.sign_in_count); // okay because the sign in count is copy trait
    // println!("{}", user2.username); // not good becuase the value is moved 
    //
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);

    let subject = AlwaysEqual; // no need curly bracket or parantheses
}


fn build_user(email: String, username: String) -> User {
    User {
        activate: true,
        username, // will assign the value to the same name field
        email,
        sign_in_count: 1
    } // here is an expression returning the instance
}
