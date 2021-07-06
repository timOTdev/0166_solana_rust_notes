// // A Move. Causes error, not a deep copy.
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }



// // A Clone. A deep copy.
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }



// // Passing ownership.
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

//     println!("{}", x);              // can still access.

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.



// // Transferring ownership with returning values and assignment.
// fn main() {
//     let _s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let _s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3

//     println!("{} {}", _s1, _s3); // Can access _s1 and _s3

// } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("hello"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }



// // Returning values as tuples.
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }



// // Using references and borrowing
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



// // Using mutable references to change the reference value.
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }



// // Optionally make a new scope to have multiple mutable references.
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
// }



// // Valid because we've already used the immuable values before we change the reference value.
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }



// // We can't return a reference of a value that will be dropped!
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!



// // This works because we are directly return String from the no_dangle function.
// fn main() {
//     let string = no_dangle();
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }



// Getting first word (Compiles but does not Error when ran)
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Running this will error!
fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}



// // The correct way to eliminate errors at compile time.
// // Better than the previous example where does not Error at compile time.
// // Does not compile to show us the error immediately.
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {}", word);
// }



// // A better way to write to accept String and str.
// // We use &str reference instead of &String.
// // Now we can take both types.
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // first_word works on slices of `String`s
//     let word = first_word(&my_string[..]);

//     let my_string_literal = "hello world";

//     // first_word works on slices of string literals
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }



// // String Slices
// fn main() {
//     let s = String::from("hello world");

//     // Method 1: Inclusive starting_index and exclusive ending_index.
//     let hello = &s[0..5];
//     let world = &s[6..11];

//     // Method 2: Not including the starting_index. Means start at 0.
//     let slice2 = &s[0..2];
//     let slice3 = &s[..2];

//     // Method 3: Dropping the ending_index.
//     let len = s.len();

//     let slice4 = &s[3..len];
//     let slice5 = &s[3..];

//     // Method 4: Dropping both starting_index and ending_index.
//     let slice6 = &s[0..len];
//     let slice7 = &s[..];

//     println!("_hello {}", _hello);
//     println!("_world {}", _world);
// }
