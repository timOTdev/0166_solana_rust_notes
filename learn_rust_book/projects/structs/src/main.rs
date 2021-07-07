// // Base Example of Struct
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// }



// // Using an instance of struct.
// // This will work and give you back the lines.
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("john@example.com"),
//         username: String::from("john doe"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user_email = user1.email;
//     let user_username = user1.username;
//     let user_active = user1.active;
//     let user_sign_in_count = user1.sign_in_count;

//     println!(
//         "{} ({}) has active state of {} and signed in {} times."
//         ,user_username
//         ,user_email
//         ,user_active
//         ,user_sign_in_count
//     );
// }



// Using build_user function.
// This won't work yet, I'm still playing around with it.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email = String::from("someone@gmail.com");

    let name = String::from("someone");

    let results = build_user(email, name);

    println!("{}", results);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
