fn main() {
    println!("Hello, world!");

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool, 
    };

    let mut user1 = User {
        email: String::from("sumeone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("email, {}", user1.email);

 
}

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
