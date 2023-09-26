struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("user email changed to: {}", user1.email);

    let user2 = build_user(
        String::from("royabhishek77@gmail.com"),
        String::from("AbRoy77"),
    );

    println!("new user \n email {}", user2.email);

    // struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // make tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,    // no need to write email: email
        sign_in_count: 1,
    }
}
