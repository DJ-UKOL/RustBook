struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn structs() {
    let mut user1 = User {
      active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@exmaple.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another3@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}