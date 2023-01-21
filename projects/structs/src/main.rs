fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@mail.com"),
        sign_in_count: 1,
    };

    println!("{:#?}", user1);

    user1.email = String::from("username@mail.com");

    println!("{:#?}", user1);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
