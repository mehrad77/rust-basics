struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("test@example.org"),
        username: String::from("cool_user"),
        active: true,
        sign_in_count: 0,
    };

    let mut _name = user1.username;
    user1.username = String::from("punk_user");

    let user2 = build_user(String::from("boo@example.org"), String::from("boomaster"));

    let user3 = User {
        email: String::from("zoo@example.org"),
        username: String::from("hayvanatbahcesi"),
        ..user2
    };

    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);
}