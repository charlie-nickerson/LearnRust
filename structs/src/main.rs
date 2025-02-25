struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let mut user1 = build_user(String::from("foo@gmail.com"), String::from("bar"));

    let user2 = User {
        active: false,
        ..user1
    };

    // println!("{user1.email}");
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email: email,
        username: username,
        sign_in_count: 1,
    }
}
