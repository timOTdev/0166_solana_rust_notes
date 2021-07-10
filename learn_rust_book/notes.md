# The Rust Programming Language

- [Learn Rust Book](https://www.rust-lang.org/learn)

# Foreword

- Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way.
- Programmers who are already working with low-level code can use Rust to raise their ambitions.
- It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write.

# Introduction

- Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.
- In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs.
- _Cargo_ is the included dependency manager and build tool.
- _Rustfmt_ is the coding style.
- _Rust Language Server_ for IDE integration.
- Big companies use Rust in production for command line tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even major parts of the Firefox web browser.
- Assumes you've written code in another language, not complete beginner to coding.
- Later chapters build on concepts in earlier chapters, and earlier chapters might not delve into details on a topic.
- Concept chapters and project chapters
- Appendices have reference-like materials

# 1.0 Getting Started

## 1.1 Installation

- I installed for windows.
- Commands in this book work in both _cmd.exe_ and PowerShell.
- `rustup update` - Updates rust to latest version
- `rustup self uninstall` - Uninstalls rust
- `rustc --version` - Check Rust version.
- `rustup doc` - Load docs in browser and read offline.

## 1.2 Hello, World

- Commands to run in terminal:

```rust
// Terminal commands
// To run style formatting.
$ rustfmt main.rs

// To compile to exe.
$ rustc main.rs

// To run the executable. Both \ and / worked for me in Windows 10.
$ ./main

// Console output.
Hello, world!

// main.rs
fn main() {
    println!("Hello, world!");
}
```

- `main` - function is the first code ran in every executable Rust program.
- `rustfmt` - formats your code. Run it on the .rs files.
- Rust indents with 4 spaces, not tabs.
- `println!` - is a Rust macro, not a function. See chapter 19.
- Needs to terminate with semicolon.
- Rust is ahead-of-time compiled, meaning someone else can run your app without installing Rust.
- `rustc` is fine for simple program compiling, use cargo for more advance dev chains.

## 1.3 Hello, Cargo

- Cargo is a build system and package manager that builds code, downloads libaries/dependencies, and builds them together.
- Cargo has the same commands for all OS
- Cargo expects your source files to live inside the src directory.
- The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
- `cargo --version` - Checks if cargo is installed.
- `cargo new hello_cargo` - Creates new cargo app like create-react-app. Will not generate Git files if an existing Git repository.
  - `cargo new --vcs=git` will generate Git files in an existing Git Repository.
- `Cargo.toml` is like package.json. Stands for Tom's Obvious, Minimal Language)

  - Packages are called _crates_ in Cargo. Like _packages_ for NPM.

    ```rust
    // Terminal commands
    // Checks if cargo is installed
    $ cargo --version

    // Creates new cargo app
    $ cargo new hello_cargo

    // Compiles cargo project to target/debug folder
    $ cargo build

    // Also works for me .\target\debug\hello_cargo.exe on Windows
    $ ./target/debug/hello_cargo

    // One command and combined cargo build + execute commands
    $ cargo run

    // Checks code compile process without creating executable
    $ cargo check

    // Compiles cargo project to target/release folder
    $ cargo build --release

    // Cargo.toml
    [package] // Section heading
    name = "hello_cargo"
    version = "0.1.0"
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]

    // To work with Rust project clones
    $ git clone example.org/someproject
    $ cd someproject
    $ cargo build
    ```

- `Cargo.lock` is generated when you run "cargo build" command and tracks dependencies versions.
- `cargo run` - doesn't show Compiling line if the files haven't changed since last time.
- `cargo check` - Help speed up development by not creating executable.
- Run `cargo build --release` when you're deploying or benchmarking.

# 2.0 Programming a Guessing Game

- Implement a random integer between 1 and 100.
- Implement the player to enter a guess.
- It will indicate too low or too high.
- If the guess is correct, it will congratulate and exit program

- Commands used:

```rust
cargo new guessing_game
cd guessing_game

cargo run

```

## 2.1 Processing a Guess

- The prelude brings only a few types into the scope of every program.
- We have to use `use` statement to bring in other libraries.
- The `std::io` is a standard library.

## 2.2 Storing Values with Variables

- Variables are immutable by default unless you put `mut`.
- `String::new` is a function that returns a new instance of String.
- `String` is a type, `::` means that `new` is an associated function of the string type.
- `new` is not implemented on the instance but rather on the String type itself.
- Other languages call this "static method".

- We could have also used `std::io:stdin` if we didn't put the `use:std::io at the top.
- The `std::io` library returns and instance of `stdin`.
- `.read_line(&mut guess)` is a method on the `stdin` and takes 1 argument.
  - It appends the user's input into a string.
  - The string argument passed in must be mutable.
- `&` indidates that the argument is a reference.
  - Lets the program access one piece of data without having to copy data into memory multiple times
  - References are immutable by default.
  - Details are not important right now, see Chapter 4.

## 2.3 Handling Potential Failure with the Result Type

- Good to divide lines of code to make it more readable.
- TIP: The right way to suppress the warning is to actually write error handling.
- But we want to crash the program, so we use `expect`.
- `.expect` is comes from a type called `io:Result`.
- There are many types that are called Result but `io:Result` here is a specific version of submodules.

- Result types are _enumerations_, and their values are called _variants_.
- There are 2 variants for Result type: Ok and Err.
- `Err` crashes the program and passes the argument to expect method.
- `Ok` will continue and return the user-inputed value (in number of bytes)

## 2.4 Printing Values with println! Placeholders

- Can also print multiple values, not use one.

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

## 2.5 Testing the First Part

- We use `cargo run` to test.
- Currently prints out the user input to the screen.

## 2.6 Generating a Secret Number

- There's no random number functionality in Rust.
- We will use `rand` crate.

## 2.7 Using a Crate to Get More Functionality

- A crate is a collection of source code files.

  - Our project is a binary crate, which has an executable.
  - A rand crate is a library crate, which has code to be used in projects.
  - You can get more crates on crates.io.
  - I think of crates like npm packages.

- We need to add the rand crate to our project.

  - We add the crate with the semantic version in the Cargo.toml file.
  - Add under the dependencies section as cargo reads from top to bottom.

- Semantic Version or SemVer is where the number `0.8.3` means at least that but below `0.9.0`.

  - This makes sure your code still functions with latest patch releases.

- We have to also `cargo build` our project to get the new crate.
  - This command grabs the data from the registry on crate.io.
  - If you run the command again, cargo is smart and will just exit with `Finished`.
  - If you make a minor code change, it will just say `compiling`.

## 2.8 Ensuring Reproducible Builds with Cargo.lock File

- The lock file ensures that the versions specified in the toml file are consistent.
- Cargo uses this lock file to build your project in the future to speed up the build process.

## 2.9 Updating a Crate to Get a New Version

- If you want to run update, run `cargo update`.

  - This command ignores the lock file.
  - This will update all the crates to the SemVer requirements.
  - Such as between `0.8.3` and `0.9.0`, it will not go beyond.
  - The updates will then be written to the lock file.

- If you want to upgrade beyond SemVer, you have to do it manually in the toml file.
  - Then run `cargo build` again to update the lock file.
  - There's more in chapter 14.

## 2.10 Generating a Random Number

- First, we add the crate: `use rand::Rng`.
- Rng defines methods that the random number generator implements.
- `rand::thread_rng` gives us the particular random number generator we are using.
- `gen_range` is the method that takes in value ranges with "..".

  - This is a method from the Rng trait we brought in for this scope.
  - Normally, lower bound inclusive and upper bound exclusive.
  - The syntax `1..101` and `1..=100` are equal in this case.

- Use `cargo doc --open` builds documentation provided by all your dependencies.
- This compiles locally and opens in your browser.
- Try clicking rand.

## 2.11 Comparing the Guess to the Secret Number

- Ordering is another enum. The variants are Less, Greater, Equal.
- `guess.cmp(&secret_number)` compares the two numbers against each other.

  - The return value is a variant of the Ordering enum.

- A `match` expression is made up of `arms`.

  - I like to think of `match` like Javascript's `switch` statment and arms are `cases`.
  - `match` identifies the first matching arm and returns that arm.

- However, the user's guess and the random number are not the same type.

  - Rust is a strong, static type system.
  - `guess` was a String instance when we declared it.
  - `secret_number` variable was originally an int when generated.
  - Rust normally defaults to `i32`, an unsigned 32-bit number.

- We have to convert the guess into an integer.

  - We use something called shadow to rename the type of the guess variable.
  - This is possible because Rust allows renaming of types when redeclared.

- We use `let guess: u32 = guess.trim().parse().expect("Please type a number");`

  - trim takes off any extra whitespace or non-numerical values like "\n".
  - When `read_line` was used, it adds the value to a new string and subsequently "\n" as well.
  - `guess` literaly is `5\n` for String.

  - `parse` will convert string into some kind of number.
    - It has to be some form of a number. Or else will return a Result type with Err variant.
    - Returns Ok variant if conversion was successful.
    - `expect` is the same as before.

## 2.12 Allowing Multiple Guesses with Looping

- Adding a loop will cause an infinite loop unless there's a stop condition.
  - We need to use `break` to stop the program.
  - If you remember, a non-integer input will also exit the program.
  - There's always Ctrl + C also.

## 2.13 Quitting After a Correct Guess

- We add a break in the "you win" arm to exit the match block.
- This will also exit the program because it is the end.

## 2.14 Handling Invalid Input

- If a user enters anything other than a number, we still want to continue.
- The way to do this is instead of using expect during the string conversion, use a `match` instead.
- Add `match` in front of the guess string conversion.

- parse method returns either Ok or Err, and also passes the value.
  - We handle the value with num which then just stores the number on the variable as normal.
  - The `_` is a catchall for any value.
  - If not a number, the loop will continue, effectively asking for another guess.

## 2.15 Summary

- We also remove the console of the secret number.
- Read on chapter 3 to learn more concepts.

# 3.0 Common Programming Concepts

## 3.1 Variables and Mutability

- Language has reserved keywords for code, also ones saved for the [future](https://doc.rust-lang.org/book/appendix-01-keywords.html).
- By default variables are immutable because easier to reason through.
- Can't change value once assigned to variable name.
- [x] Variables Exercise
- `mut` in front of variable name allows you to make variable mutable.
- Balance performance versus clarity.
  - Big data structures are faster if you mutate values.
  - Small data structures are easier to abstract and write functional programming style.
- Values and constants are different.

- Constant

  - Uses `const` keyword. Diferences from let:
    1. Are _always_ immutable. You can't use `mut` with constants.
    2. Can be declared in any scope, including global.
    3. Set by constant expression, not result of a function call or computed values.
  - Convention to use all caps. IE `const MAX_POINTS: u32 = 100_000`
  - Constants last the lifetime for the whole of the program.
  - Scoped to where declared.

- Shadowing

  - When values carry over in subsequent declarations.
  - We are just transforming the values but we can't reassign the variable name without using `let` keyword.
  - Also allows changing of the type of the variable because we are _effectively creating a new variable with every new shadow._

    ```rust
    // Shadowing.
    fn main() {
        let x = 5;

        let x = x + 1;

        let x = x * 2;

        println!("The value of x is: {}", x);
    }
    // => The value of x is: 12

    // Changing types with shadowing is fine.
    fn main() {
        let spaces = "   ";
        let spaces = spaces.len();
    }

    // Changing types with mut not allowed.
    fn main() {
        let mut spaces = "   ";
        spaces = spaces.len();
    }
    ```

## 3.2 Data Types

- Rust is a _statically typed language,_ we must know types of all variables at compile time.
- Compiler usually infers types from the value initialized for the variable.

```rust
  fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // You can't leave type empty.
    // let guess = "42".parse().expect("Not a number!");
  }
```

- There are 2 data type subsets: scalar and compound.
- Scalar types represent a single value.
  - Rust four primary scalar types integers, floating-point numbers, booleans, and characters.
- Compound types can hold multiple values such as tuples and arrays.

- Integer Types

  - Can be signed and unsigned.
  - Rust's integer defaults to `i32` , generally the fastest even on 64-bit systems.
  - Use `i` for signed integer types
    Use `u` for unsigned interge types
    Number represents amount of bits.
    8, 16, 32, 64, 128, arch bits.
  - u8 is unsigned 8 bit integer.
  - `isize` or `usize` are arch bits and depends on your computer architecture. Use these when indexing some sort of collection.

  - You can write in many ways including decimal, hex, octal binary, and byte.
  - Numbers over the bit settings will cause _integer overflow_ and _panic_ at runtime (program exits with an error.
  - During —release flag, rust does _not_ check for integer overflow that causes panic.

  - _Complement Wrapping_ will circle back to start at the minimum value of the range.
  - There's 4 types to handle this overflow.
    1. wrapping\_\* wraps in all modes IE `wrapping_add`
    2. checked\_\* - Returns None value if overflow
    3. overflowing\_\* - Returns value and Boolean indicating overflow
    4. saturating\_\* - saturate with the minimum or maximum value.

- Floating-Point Types

  - Uses `f` character.
  - Are decimal numbers and can be f32 or f64, default is f64.
  - f64 has double precision and roughly same speed as f32.

    ```rust
    fn main() {
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32
    }
    ```

- Numeric Operations

  - We got all the basic math operations.

    ```rust
    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;

        // remainder
        let remainder = 43 % 5;
    }
    ```

  - Boolean Type
    - Either `true` or `false` and take up 1 byte.

- Character Types

  - Use single quotes with char literals.
  - Use double quotes for string literals.
  - 4 bytes in size and can be more than just ASCII.

- Compound Types

  - Structures that can hold multiple values into 1 type.
  - Rust has tuples and arrays.

  - Tuple

    - Tuples can vary with data types.
    - Tuples are fixed length, can't redeclare size.
    - Can destructure tuples into variables.
    - Index access uses `.` IE x.0 or x.1
    - Rust uses zero-based indexing.

    ```rust
    // Heterogenous collection
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }

    // Tuple destructuring
    fn main() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {}", y);
    }

    // Accessing by index (zero based)
    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }
    ```

  - Array

    - Must all be of the same type.
    - Have fixed lengths like Tuples.
    - Most other languages have mutable array lengths, _not_ Rust.
    - Vectors are homogenous types but can change size.
    - Useful when allocate to stack rather than heap.

    ```rust
    // Standard array initialization
    fn main() {
        let a = [1, 2, 3, 4, 5];
    }

    // Setting the type and length of array
    fn main() {
      let a: [i32; 5] = [1, 2, 3, 4, 5];
    }

    // Alternate array initialization
    let a = [3; 5];
    let a = [3, 3, 3, 3, 3]; // The same as above

    // Collection of known length, use array instead of vector.
    fn main() {
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    }

    // Array access
    fn main() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }
    ```

  - Will get out of bounds if accessing outside of range. (runtime error)
  - Rust's first example of safety principle: will check the index the user enters and exits program (aka panic). Other low-level languages don't do this check and let you access that invalid physical memory space and continue running.

## 3.3 Functions

- `main` is the primary entry point of a program.
- Rust uses snake case all lowercase and with underscores.
- Function location doesn't matter, it will get hoisted.
- Function parameter types must always be defined. Helps for compiler to discern any ambiguity.

```rust
// Function examples
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

// See more examples in projects.
```

- Rust is an expression-based language, more distintive versus other languages.
- _Statements_ are instructions that perform some action and do not return a value.
- _Expressions_ evaluate to a resulting value.

  - Also calling a function, calling a macro, or creating new scopes with {} are examples of expressions.

- You can't assign a value to a variable if statements never return a value.
- Rust is different from Ruby and C where assignment returns the value of the assignment.

```rust
// Function definitions are also statements in itself.
fn main() {
    let y = 6;
}

// Error: There's no value being returned for the variable to bind to.
fn main() {
    let x = (let y = 6);
}

// Ruby and C act like this, not Rust.
// Now both x and y each equal 6.
fn main() {
    x = y = 6;
}
```

- The Y block is an expression because it returns 4.
- _Expressions do not include ending semicolons_.
- If you add a semicolon, it is considered a statement and will not return a value.

```rust
// x + 1 is an expression here and does NOT have a semicolon.
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

- We declare types of the return value but don't name the return values.
- Just the number 5 is perfectly valid function in Rust.
  - This is because there's no semicolon so considered and expression, aka it returns that value.

```rust
// The type of return is defined as i32.
// Note: no semicolon on 5 makes it an expression.
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

// x +1 will be evaluated and return.
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// This will error because of semicolon on x+1;
// That makes it a statement, not an expression.
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

## 3.4 Functions

- Simple comments use `//`
- For mutiple comments, each line needs double slashes.
- Convention is usually have d-slashes above when you want to comment.
  - Possible but not convention: doing inline double slashes to make comments.

```rust
// Normal comment
fn main() {
// hello, world
}

// Multi-line comment
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}

// Convention to comment above the line.
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}

// Possible but not recommended.
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

## 3.5 Control Flow

- If statements don't have the parentheses.
- Else blocks of code are optional just like other languages.

- The conditional in an if-statement must be a boolean.
- Unlike Ruby or JS, Rust will not auto try to convert non-boolean types.

- Rust evaluates the first true condition and stops.
- Chapter 6 has `match` construct for too many if statements.

```rust
// This will cause an error.
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

// The statement evaluates to boolean, so this works.
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

// Only the 2nd block was ran, being the first true condition.
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

- The if-else arms must have the same type of will give an error.
- Rust must know the type of the variable at compile time to guarantee type safety.
- Determining type at runtime is mor complex.

```rust
// "six" needs to be 6 to evaluate correctly.
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

- Loops

  - 3 types: `loop`, `while`, `for`
  - `break` lets you exit the loop.
  - Can also supply value to return after break expression.
  - Notice the ; to end the statement and assigns the value to result.

    ```rust
    // This runs forever until you push Ctrl + C.
    fn main() {
        loop {
            println!("again!");
        }
    }

    // If you want a return value to know when the loop broke.
    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }
    ```

- While
  - Runs until the condition is false.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

- For loops
  - The most commonly used loops in Rust.
  - Can also use a while loop to iterate over a collection. This is more error prone if index length is wrong.
  - Also slower because it has to check conditionals at every iteration.
  - `Range` is a useful type provided with the standard library if you want to do a countdown.
  - `rev` also provides a reversed range.

```rust
// You won't have to track index length.
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// Using Range.
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

- Summary
  - You can build small programs to learn:
    1. Convert temperatures between Fahrenheit and Celsius.
    2. Generate the nth Fibonacci number.
    3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

# 4.0 Understanding Ownership

- Allows Rust to make memory safety guarantees without needing a garbage collector.

## 4.1 What is Ownership?

- All programs have to manage computer's memory while running.

  - Some programs do it while the program is running or the programmer must do it manually.
  - Rust has ownership with rules that the compiler checks at compile time.
  - Ownership is a new concept so spend time to understand and to get good at it.

- **Stacks and Heaps**

  - Stacks follow LIFO.
  - We _push_ on to the stack and _pop_ off the stack.
  - The data stored on stacks must have a known, fixed size.
  - The anology is like stacking plates.

- **Heaps**

  - Heaps follow FIFO.
  - Heaps are less organized because of the data size variability.
  - Space must be requested to fit the data then a pointer must be returned.
  - _Allocating on the heap_ is when you do this assignment process.
  - The analogy is like being seated at a restaurant.

- **Heaps shortcomings**

  - Heaps are slower to allocate the data, especially if the data size is big.
  - Slow to access the data because it has to use the pointer to get there.
  - It has to bookkeep to prepare for the next allocation.
  - It must make sure there are no duplicate data, freeing up unused data on the heap, allocating new space if needed.
  - Ownership aims to solve all the problems of the heap.

- **Ownership Rules**

  - Each value has a variable called its _owner_.
  - There can only be one owner at a time.
  - The value is dropped when the owner goes out of scope.

- **Variable Scope**
  - The idea of when _owner_ is in scope and not.

```rust
fn main() {
    // s is not valid here, it’s not yet declared

    let s = "hello";   // s is valid from this point forward

    // do stuff with s

    // this scope is now over, and s is no longer valid
}
```

- **The String Type**

  - We need to use a type more complex than chapter 3's string literal.
  - Strings, the type, are not known at compile time, so we need a way to allocate enough space.
  - String literals are immutable because how it deals with memory.
  - We use the second string type, `String`. This is allocated to a heap.
  - Creating a `String` from a string literal: `let s = String::from("hello");`

- We can mutate this String type now after converting from string literal to the String type.
- Remember that string literals are immutable at compile time.

```rust
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
```

- **Memory and Allocation**

  - Again, string literals are fast because they are known before compile time and are immutable.
  - We need to use the `String` type for unknown data.

  - It needs to be requested from memory at runtime and the memory needs to be returned with we're done.
  - Rust automatically removes memory allocation at the end of the closing curly brace by calling `drop`.

  - Historically, other languages with garbage collector (GC), we have to pair an _allocate_ in exact amounts of _free_.
  - In C++, this pattern is called _Resource Aquisition Is Initialization_ (RAII).

- **Ways Variables and Data Interact: Move**

  - The basic string literal is easy because it is a copy by value, not by reference.
  - A String type is a copy by reference in my understanding.

  - The pointer, length, and capacity is actually being copied from the String type of "hello". This data group is stored on the stack.
  - It can be though of as a _shallow copy_, or a _move_ in Rust terms.
  - Not actually copying the values of the String "hello" to a new memory block (aka _deep copy_).
  - The code below will cause an error because of _double free error_.

  - Rust will never make deep copies unless you tell it to.
  - Any "automatic copies" are assumed to be inexpensive for runtime performance.

```rust
fn main() {
    // Move example
    // A shallow copy which is inexpensive.
    // let x = 5;
    // let y = x;

    // String Example
    // s1 was moved into s2.
    // Will cause error because s1 was borrowed.
    // Error: move occurs because `s1` has type `String`, which does not implement the `Copy`
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

- **Ways Variables and Data Interact: Clone**

  - Using `clone` will make a deep copy.
  - Considered expensive operation.
  - This is what we actually wanted to do.

  ```rust
  // Clone is a deep copy.
  fn main() {
      let s1 = String::from("hello");
      let s2 = s1.clone();

      println!("s1 = {}, s2 = {}", s1, s2);
  }
  ```

- **Stack-Only Data: Copy**

  - We can use _Copy_ trait for types like integers that need to go on the stack.
  - The following types implement the _Copy_ trait and is still usuable after the assignment.
  - Types:
    1. All the integer types, such as u32.
    2. The Boolean type, bool, with values true and false.
    3. All the floating point types, such as f64.
    4. The character type, char.
    5. Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

- **Onwership and Functions**
  - Notice a `move` has a `drop` called later
  - The main takeaway is that things go out of scope when ownership is passed.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is a Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

- **Return Values and Scope**
  - You can also pass ownership by returning it and assigning it to a variable.
  - You can also use tuples to return back data.
  - Tuples are not the best way as we'll learn later.
  - Items only go out of scopes if ownership is not moved.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

- Using tuples in return data.
- We have something easier for all this called `references`.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

## 4.2 References and Borrowing

- Now we are going to use _references_ by using ampersands `&`.
  - _references_ are refering to some value without taking ownership of it.
  - _dereferencing_ uses `*`. See chapter 8 and 15.
  - In the previous section code, we have to return the variable to pass the ownership back to the variable to be used in the string println.
  - Now this new code with reference is different.
    - We don't have a tuple for the variable name.
    - We pass in `&s1` to the `calculate_length` function. The function takes in `&String`.
    - The return in the function is gone also.

```rust
fn main() {
    let s1 = String::from("hello");

    // &s1 refers to the value of s1 but does NOT own it.
    // So, s1 will not be dropped.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// & in String indicates that s is a reference.
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- **Borrowing**
  - This is the idea of having references as function parameters is _borrowing_.
  - One can't modify something we are borrowing either!
  - Like _variables_ are immutable by default, _references_ are also immutable.
  - However, there is only 1 way to get around this.

```rust
// Error: you can't modify a reference you borrow!
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

- **Mutable References**
- We can mutate the reference by using `&mut`.
- Can only have max one mutable reference per scope!

```rust
fn main() {
    let mut s = String::from("hello");

    // Add &mut to make the reference mutable.
    change(&mut s);
}

// Also have to let the function know that it's mutable.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- There can only be one mutable reference to a piece of data in a particular scope.
- This restriction helps keep the mutation handled in a controlled fashion.
- It helps prevent _data race conditions_ because Rust doesn't even compile code with data races.
  1. When 2 pointers or more try to access the same data.
  2. When 1 of the pointer is being written with data.
  3. No mechanism to synchronize access to the data.

```rust
// Example 1 Error
fn main() {
    let mut s = String::from("hello");

    // Can't have 2 mutable references in a scope!
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

// Example 2 Error
fn main() {
    let mut s = String::from("hello");

    // Users of an immutable reference don’t expect the values to suddenly change out from under them!
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM: we change the reference right before we print out the value.

    println!("{}, {}, and {}", r1, r2, r3);
}
```

- However, this is okay because we use the data before we try to mutate it.
- Or we can give it its own scope.

```rust
// Use the reference before mutating it is okay.
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// Making own scope.
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

- **Dangling References**
- A _dangling pointer_ is where the pointer to the location in memory may be given to something else. Basically, pointing to something that was freed from memory or something we are not expecting.
  - Rust also guarantees that the data will not go out of scope before the reference to the data does.
  - There's something called _lifetime_ but that's in chapter 10.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // new String

    &s // return a reference to the String, s

    // s will be dropped after the curly brace,
    // the reference to s will be pointing to invalid String. Danger!
}
```

- Instead we want to return the String directly.
- The ownership is moved outside of the function.

```Rust
fn dangle() -> String {
    let s = String::from("hello");

    s
}
```

-**Summary**

- You can have unlimited immutable references but _only_ one mutable reference.

- References must always be pointed correctly and valid.

## 4.3 The Slice Type

- _slice_ allows referencing of a contiguous sequence of elements in a collection.

- The main problem with this example is that `word` will change after we do the `s.clear()`.
- In other words, the index in `word` might get out of sync with the data in `s`.
- Even though the value 5 still exists for `word`, it doesn't mean anything if `s` is now equal to "".
- There's nothing useful we can do with the value.

```rust
// We want to reference only the first word in the string.
// &String is a reference to the position allocation of the string.
// usize is used for index in a collection.
fn first_word(s: &String) -> usize {
    // Need to convert the string into array of bytes so we can loop.
    let bytes = s.as_bytes();

    // Loops through the bytes using iter() method.
    // enumerate() wraps results of each iteration and returns as a tuple.
    // i is the index and &item (a single byte) is the reference to the element.
    for (i, &item) in bytes.iter().enumerate() {
        // Now we search each item if it's an empty space, ' '.
        // The b represents byte?
        // Return the index of the empty space.
        // Counts from zero index.
        if item == b' ' {
            return i;
        }
    }

    // If nothing is found, return the string length.
    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

- **String Slices**

- Allows you to slice the strings in different ways.
- For this chapter, we assume that the string has valid UTF-8 character boundaries.
- If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

```rust
fn main() {
    let s = String::from("hello world");

    let len = s.len();

    // Method 1: Inclusive starting_index and exclusive ending_index.
    let hello = &s[0..5];
    let world = &s[6..11];

    // Method 2: Not including the starting_index. Means start at 0.
    let slice = &s[0..2];
    let slice = &s[..2];

    // Method 3: Dropping the ending_index.
    let slice = &s[3..len];
    let slice = &s[3..];

    // Method 4: Dropping both starting_index and ending_index.
    let slice = &s[0..len];
    let slice = &s[..];
}

```

- This is the best way to write the method.
  - We use `&str` reference instead of &String and being able to take both String and str.
  - `&s[0..i]` gives you a slice of the string instead of just an index like before.
  - `&my_string[..]` is where we pass in the a slice (effectively a copy) of the whole string.
  - Now we don't worry about the reference, `my_string`, changing or getting out of sync.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

- **Other Slices**

- We can also slice arrays like so:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

- **Summary**

- Using ownership, borrowing, and slices helps ensure memory safety in Rust programs at compile time.

# 5.0 Using Structs to Structure Related Data

- A _struct_ or structure is a custom data type that lets you name and package together multiple related values that make up a meaningful group.

## 5.1 Defining and Instantiating Structs

- _structs_ are like C# or Javascript classes to me. You can also think of it as a template or scaffold.
- _fields_ are like the members of a class to me.
- We make _instances_ of a struct to use it.
  - Don't have to specify the fields in the same order as in the struct.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

- **Changing specific values of Struct**

  - Use dot notation.
  - All the fields are mutable by default, you can't select some to be mutable and others immutable.

```rust
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

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

- **Field Init Shorthand**

  - Fields are like in javascript where you don't have to repeat the key and the value name.
  - Write `email` rather than `email: email`.
  - Just write it once:

  ```rust
    // Shorthand where we don't have to write key and value names.
    // It's inferred.
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
  ```

- **Struct Update Syntax**

  - Use the same fields from user1, now in user2

  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  fn main() {
      let user1 = User {
          email: String::from("someone@example.com"),
          username: String::from("someusername123"),
          active: true,
          sign_in_count: 1,
      };

      // Notice how we can use user.active and user1.sign_in_count here.
      let user2 = User {
          email: String::from("another@example.com"),
          username: String::from("anotherusername567"),
          active: user1.active,
          sign_in_count: user1.sign_in_count,
      };

      // Even cleaner using syntax `..`
      // Has the same values for the active and sign_in_count fields from user1
      let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
  }
  ```

- **Using Tuple Structs without Named Fields to Create Different Types**

  - _Tuple structs_ are like structs but just has the types of the fields and not the member key name.
  - We still have to use two separate structs here even though the field pattern of `i32` is the same in both.
  - Just useful when we want to make different struct types but just have different names.
  - You can't substitute Color and Struct, they are different!
  - Use dot notation to access field values.

  ```rust
  fn main() {
      struct Color(i32, i32, i32);
      struct Point(i32, i32, i32);

      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0);
  }
  ```

- **Unit-Like Structs Without Any Fields**

  - _Unit-like structs_ are structs that don't have any fields.
  - Useful to implement trait on some type but don't have any data to store in the type itself. Don't understand this yet...see chapter 10.

- **Ownership of Struct Data**

  - This example won't work without understanding lifetimes.
  - _Lifetimes_ ensures that the data lasts as long as the struct does.
  - Using `&str` means that the `User` struct doesn't own all of its data, it has references to another location.
  - For now, we should just use `String` type until we learn later how to use `&str` in our structs.

    ```rust
    struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
    }

    fn main() {
        let user1 = User {
            email: "someone@example.com",
            username: "someusername123",
            active: true,
            sign_in_count: 1,
        };
    }
    ```

## 5.2 An Example Program Using Structs

- We want to use struct to give more clarity to our code.
- Using multiple arguments or a tuple to an function argument is suffcient.
- We can do better, use a struct.

- This is the basic way to write our feature.
- Doesn't tell you about the relationship of the 2 variables and how it forms a rectangle.

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- **Refactoring with Tuples**

  - Tuples allows a pairing of the width and length value but there are some confusing parts.
  - Don't know which index in the tuple represents which variable.
  - Accessing index 0 and 1 is not very helpful either.

    ```rust
    fn main() {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    ```

- **Refactoring with Structs: Adding More Meaning**

  - This is the best way to write it.

    ```rust
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        // We are using an immutable borrow of the Rectangle struct.
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    ```

- **Adding Useful Functionality with Derived Traits**

  - How do we see the struct we created?
  - Use `{:?}` or `{:#?}` with annotation `#[derive(Debug)]`
  - `{:?}` - normal print
  - `{:#?}` - pretty print
  - println! macro won't work here because only work on primitives.
  - We also have to opt into the debug functionality with the annotation.

    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {:?}", rect1);
    }
    ```

## 5.3 Method Syntax

- Methods are basically functions within a struct.
- It helps with organization because all methods implemented under one `impl` block.

- **Defining Methods**

  - Methods can take ownership of `self` or borrow `self` mutably.
  - If we generally want to read data, just use `&self` reference.

```rust
#[derive(Debug)]
// We make a struct.
struct Rectangle {
    width: u32,
    height: u32,
}

// Then we add a method onto the struct.
// Use impl, aka implementation, to add the method.
impl Rectangle {
    // &self refers to Rectangle struct.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Calls the area method from the rect1 struct.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- **Where's the -> Operator?**

  - Rust doesn't have `.`or `->` where we have to dereference the pointer first.
  - Rust has _automatic referencing and dereferencing_ where it figures out to automatically add `&`, `&mut`, or `*`.
  - Automatic referencing works because methods have a clear receiver, the type of `self`.

  ```rust
  // These are equal. First is cleaner.
  p1.distance(&p2);
  (&p1).distance(&p2);
  ```

- **Methods with More Parameters**

  - We implement 2 methods here: area and can_hold.
  - can_hold takes on more arguments than just `&self`.

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }

      // Can take in additional arguments.
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }

  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
      let rect2 = Rectangle {
          width: 10,
          height: 40,
      };
      let rect3 = Rectangle {
          width: 60,
          height: 45,
      };

      println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
      println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  }
  ```

- **Associated Functions**

  - Often used for constructors.
  - Associated Functions don't take a `self` as a parameter and "associated" with the struct.
  - So it doesn't need to access the data on the struct at all.
  - Accessed through `::` syntax like `Rectangle::square(3);`

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Rectangle {
              width: size,
              height: size,
          }
      }
  }

  fn main() {
      let sq = Rectangle::square(3);
  }

  ```

- **Multiple impl Blocks**

  - It's valid syntax to have 2 separate impl blocks but not common practice.
  - It's more efficient just to combine the 2 methods under one `impl` block.

# 6.0 Enums and Pattern Matching

- _Enum_ allow you to make a type with all its possible data types as variants.
- This helps give meaning with your data.
- _Option_ lets you express a value that is something or none.
- Enums in Rust are similar to algebraic data types in functional languages like F#, OCaml, and Haskell.

## 6.1 Defining an Enum

- **Defining an Enum**

  - We can specify 2 specific types like in case of IP addresses: V4 or V6.
  - Its variants can only be one of the two.
  - Using enums means we are making a "custom struct" but with more flexibility.

    ```rust
    fn main() {
      // IpAddrKind is termed the kind.
      enum IpAddrKind {
        // Two options are termed variants.
          V4,
          V6,
      }

      // A struct can incorporate an enum too.
      struct IpAddr {
          kind: IpAddrKind,
          address: String,
      }

      // Use :: to specify the variant of the enum.
      let home = IpAddr {
          kind: IpAddrKind::V4,
          address: String::from("127.0.0.1"),
      };

      let loopback = IpAddr {
          kind: IpAddrKind::V6,
          address: String::from("::1"),
      };
    }
    ```

    - Here is the same code but more concise and exactly the same features.
    - This helps eliminate the extra struct.

    ```rust
    fn main() {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }
    ```

- **Enum Values**

  - Enums can also have variants of different types and amounts of data.
  - You can put in strings, numerics, structs, and even other enums.

    ```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // The standard library already has IP4 and IP6 variants since it's so common.
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    ```

  - You can write enums just like struct.
  - Enums can also have methods just like struct's.
  - This example didn't compile for me. What's suppose to be in the call() body?

    ```rust
    // Written as enums.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // The same thing written as struct's
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // Method implementation just like with struct's.
    fn main() {
      enum Message {
          Quit,
          Move { x: i32, y: i32 },
          Write(String),
          ChangeColor(i32, i32, i32),
      }

      impl Message {
          fn call(&self) {
              // method body would be defined here
          }
      }

      let m = Message::Write(String::from("hello"));
      m.call();
    }
    ```

- **The Option Enum and Its Advantages Over Null Values**

  - Many languages have `null`, which means there's no value there.
  - Rust does not have null but uses Option Enum instead.
  - I think of Option Enum like a built in null type checker so we don't have to run null checks.
  - We know there's a value and it's not null.

  - The standard library has `Option<T>` and so common that it's included in the prelude by default.
  - `<T>` just means the generic type. More on chapter 10.
  - FYI, you have to extract/convert `T` from the Option before you can use it.

  - Option Enum has only `Some` or `None` variants.
  - `Some` just means there's a value and it's held inside.
  - `None` means there is no value, basically null.

  - The ways to convert `Option<T>` to `T` are numerous. Check the documentation.
  - But we need to handle how to handle different types of the variants.
  - Using `match` or `if let` are good ways to do this.

  ```rust
  // This will error because Rust doesn't know
  // how to add i8 + Option<i8>.
  fn main() {
      let x: i8 = 5;
      let y: Option<i8> = Some(5);

      // You have to convert Option<i8> to i8 first.
      let sum = x + y;
  }
  ```

## 6.2 The match Control Flow Operator

- Our coin will either be the 4 types of variants for the enum Coin.
- Variant arms are usually brief but we can use curly braces to use more code.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // If the argument coin passed in matches any of these variant arms, it will run the code to the right of =>.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

- **Patterns that Bind to Values**
  - We can have enums inside of enums and will handle specific cases like Alaska state.

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Enum inside of existing enum.
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

- **Matching with Option<T>**
  - We use enums to also check argument types and handle accordingly.

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

- **Matches are Exhaustive**

- We need to handle all cases.
- If you don't exhaust all the possibilities, Rust won't compile the code.

```rust
// This will error.
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // Not exhaustive enough.
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

- **The \_ Placeholder**

  - Use `_ => ()` to capture all types and return a unit.
  - Underscore just matches all cases and unit doesn't do anything.

```rust
// Example of covering all possibilities.
// However, too exhaustive for match, use if let instead.
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```

## 6.3 Concise Control Flow with if let

- `if let` is syntactic sugar for `match` if you want to handle only 1 case.
- It's less verbose and you can also include an `else` statement.
- We lose out on exhaustive checking for conciseness.

```rust
// The standard match flow.
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

// Using if let is cleaner.
fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```

- Here's another example where we can add `else` as well.

```rust
// Example where we were using match for state coins.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

// Same example written with if let and else statements.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// The syntax takes getting use to.
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```

# 7.0 Managing Growing Projects with Packages, Crates, and Modules

## 7.1 Packages and Crates

## 7.2 Defining Modules to Control Scope and Privacy

## 7.3 Paths for Referring to an Item in the Module Tree

## 7.4 Bringing Paths Into Scope with the use Keyword

## 7.5 Separating Modules into Different Files
