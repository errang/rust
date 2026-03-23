struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@gmail.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user.username);
}
