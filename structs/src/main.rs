struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
     
}

fn user_ops() {
    let mut user1 = User {
        email: String::from("something@gmail.com"),
        username: String::from("something"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    user1.username = String::from("something2");

    let user2 = build_user(
        String::from("blablabla@gmail.com"),
        String::from("blablabla"),
    );

    let user3 = User {
        email: String::from("hahaha@gmail.com"),
        username: String::from("hahaha"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
