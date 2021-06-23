fn main() {
    let user1 = User {
        username: String::from("Alice123"),
        email: String::from("alice@tutanota.com"),
        active: true,
        sign_in_count: 5,
    };

    println!("Username: {}", user1.username);

    let user2 = User {
        active: false,
        ..build_user(String::from("sophia@protonmail.com"), String::from("Sophia123"))
    };

    println!("User {} is active = {}", user2.username, user2.active);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

struct Colour(u32, u32, u32);
struct Point(i32, i32, i32);