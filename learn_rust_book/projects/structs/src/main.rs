// Base Example of Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}



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



// // Using build_user function.
// // Have to use pretty print with debug.
// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let email = String::from("someone@gmail.com");

//     let name = String::from("someone");

//     let results = build_user(email, name);

//     println!("{:?}", results);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }



// // Using basic arguments for Rectangle area.
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }



// // Using tuples for Rectangle area.
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }



// // The best way: structs gives meaning and relationship
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// // Printing out Struct values
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:#?}", rect1);
// }



// // Defining a method on a struct.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }



// // Having more than one parameter for a method.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }



// // Associated functions, not methods, are for constructors
// // and return a new instanace of the struct.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let sq = Rectangle::square(3);
// }
