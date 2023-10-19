struct User {
    // struct
    activate: bool, // field
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        username: String::from("test"),
        activate: true,
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };
    user1.sign_in_count += 1;

    println!("{}", user1.sign_in_count);

}
