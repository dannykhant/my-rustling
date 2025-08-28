struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);

    let subj = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1
    }
}
