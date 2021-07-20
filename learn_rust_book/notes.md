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
  1. `cargo new guessing_game`
  2. `cd guessing_game`
  3. `cargo run`

## 2.1 Processing a Guess

- The prelude brings only a few types into the scope of every program.
- We have to use `use` statement to bring in other libraries.
- The `std::io` is a standard library.
- We're in the guessing_game folder.

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
      println!("Please input your guess.");

      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
            println!("You win!");
            break;
          },
      }
    }
}
```

## 2.2 Storing Values with Variables

- Variables are immutable by default unless you put `mut`.
- `String::new` is a function that returns a new instance of String.
- `String` is a type, `::` means that `new` is an associated function of the string type.
- `new` is not implemented on the instance but rather on the String type itself.
- Other languages call this a "static method".

- We could have also used `std::io:stdin` if we didn't put the `use:std::io` at the top.
- The `std::io` library returns and instance of `stdin`.
- `.read_line(&mut guess)` is a method on the `stdin` and takes 1 argument.
  - It appends the user's input into a string.
  - The string argument passed in must be mutable.
- `&` indidates that the argument is a reference.

  - Lets the program access one piece of data without having to copy data into memory multiple times.
  - References are immutable by default. More in Chapter 4.

## 2.3 Handling Potential Failure with the Result Type

- Good to divide lines of code to make it more readable.
- TIP: The right way to suppress the warning is to actually write error handling.
- But we want to crash the program, so we use `expect`.
- `.expect` is a variant that comes from a type called `io:Result`.
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

- A *crate* is a collection of source code files.

  - Our project is a *binary* crate, which has an executable.
  - A rand crate is a *library* crate, which has code to be used in projects.
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
- Cargo uses this lock file to intelligently build your project in the future to speed up the build process.

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
  - The syntax `1..101` and `1 .. =100` are equal in this case.

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

- We have to convert the guess, a string, into an integer.

  - We use something called "shadow" to rename the type of the guess variable.
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

- **Constant**

  - Uses `const` keyword. Diferences from let:
    1. Are _always_ immutable. You can't use `mut` with constants.
    2. Can be declared in any scope, including global.
    3. Set by constant expression, not result of a function call or computed values.
  - Convention to use all caps. IE `const MAX_POINTS: u32 = 100_000`
  - Constants last the lifetime for the whole of the program.
  - Scoped to where declared.

- **Shadowing**
  - When values carry over in subsequent declarations by redeclaring the variable with `let`.
  - We are just transforming the values but we can't reassign the variable name.
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
- Sometimes we have to explicitly tell it what type.

```rust
  fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // You can't leave type empty.
    // let guess = "42".parse().expect("Not a number!");
  }
```

- **Data Types**
- There are 2 data type subsets: scalar and compound.
- Scalar types represent a single value.
  - Rust four primary scalar types integers, floating-point numbers, booleans, and characters.
- Compound types can hold multiple values such as tuples and arrays.

- **Integer Types**
  - Can be signed and unsigned.
  - Rust's integer defaults to `i32` , generally the fastest even on 64-bit systems.
  - Use `i` for signed integer types
  - Use `u` for unsigned interge types
  - Number represents amount of bits.
  - 8, 16, 32, 64, 128, arch bits.
  - u8 is unsigned 8 bit integer.
  - `isize` or `usize` are arch bits and depends on your computer architecture. Use these when indexing some sort of collection.

  - You can write in many ways including decimal, hex, octal binary, and byte.
  - Numbers over the bit settings will cause _integer overflow_ and _panic_ at runtime (program exits with an error.
  - During `—release` flag, rust does _not_ check for integer overflow that causes panic.

- **Integer overflow and complement wrapping**
  - Integer overflow is when numbers go beyond their designated bits.
  - C languages have been known to do *complement wrapping*.

  - _Complement Wrapping_ will circle back to start at the minimum value of the range.
  - There's 4 types to handle this overflow.
    1. wrapping\_\* - wraps in all modes IE `wrapping_add`
    2. checked\_\* - Returns None value if overflow
    3. overflowing\_\* - Returns value and Boolean indicating overflow
    4. saturating\_\* - saturate with the minimum or maximum value.

- **Floating-Point Types**
  - Uses `f` character.
  - Are decimal numbers and can be f32 or f64.
  - Default assignment is f64.
  - f64 has double precision and roughly same speed as f32.

    ```rust
    fn main() {
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32
    }
    ```

- **Numeric Operations**
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

  - **Boolean Type**
    - Either `true` or `false` and take up 1 byte.

- **Character Types**
  - Use single quotes with char literals.
  - Use double quotes for string literals.
  - 4 bytes in size and can be more than just ASCII.

- **Compound Types**
  - Structures that can hold multiple values into 1 type.
  - Rust has tuples and arrays.

  - **Tuple**
    - Tuples can vary with data types.
    - Tuples are fixed length, can't redeclare the size.
    - Can destructure tuples into variables.
    - Index access uses `.` IE x.0 or x.1
    - Rust uses zero-based indexing with `.`.

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

  - **Array**
    - Must all be of the same type. This is opposite of *tuples* with heterogenous types.
    - Have fixed lengths like *tuples*.
    - Most other languages have mutable array lengths, _not_ Rust.
    - Vectors are homogenous types but can change size.
    - Useful when allocate to stack rather than heap.
    - We have array access put uses brackets `[]`.

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

- `main` is the primary entry point of every Rust program.
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
// It will return the value 4 to y variable.
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
//==The type of return is defined as i32.
// Note: no semicolon on 5 makes it an expression.
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

//==x +1 will be evaluated and returned.
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//==This will error because of semicolon on x+1;
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
- Convention is usually have d-slashes with comments, above where you want to comment to point to.
  - Possible but not convention: doing inline double slashes to make comments at the end of the statement.

```rust
//==Normal comment
fn main() {
// hello, world
}

//==Multi-line comment
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}

//==Convention to comment above the line.
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}

//==Possible but not recommended.
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
//==This will cause an error. 3 is not "truthy".
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

//==The statement evaluates to boolean, so this works.
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

//==Only the 2nd block was ran, being the first true condition.
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
//=="six" needs to be int 6 to evaluate correctly.
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

- **Loops**
  - 3 types: `loop`, `while`, `for`
  - `break` lets you exit the loop.
  - Can also supply value to return after break expression.
  - Notice the `;` to end the statement and assigns the value to result.

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

- **While**
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

- **For**
  - The most commonly used loops in Rust.
  - Can also use a while loop to iterate over a collection. This is more error prone if index length is wrong.
  - Also slower because it has to check conditionals at every iteration.
  - `Range` is a useful type provided with the standard library if you want to do a countdown.
  - `rev` also provides a reversed range.

```rust
//==You won't have to track index length.
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

//==Using Range.
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

- We can divide the program into many parts.
- We also encapsulate implementation detail and can use it from other parts of the program.
- We can decide to make parts public and private.
- We have to be mindful of scopes and variable names that are in and out of scope.
- We need to keep in mind which variables are what types like function, struct, enum, etc.

- **The Module System**
  1. Packages
  2. Crates
  3. Modules and use
  4. Paths

## 7.1 Packages and Crates

- **A package must contain zero or one library crate, no more. You can have as many binary crates but need at least one crate (binary or library).**

- The _crate root_ is the source file that the Rust compiler starts from.
- _Crates_ are binary or library that has a crate root.

  - `main.rs` is an example of a binary crate and considered the current crate root.
  - `lib.rs` is considered a library and so this becomes the crate root over `main.rs`, if both are present.

- _package_ is one or more crates that provide a set of functionality. It has a _cargo.toml_ file.

  - A package can have multiple binary crates by putting the files in `src/bin` folder.

- So the `rand` crate has an `Rng` trait but we can also define a struct named `Rng`.
  - To avoid confusion, we can access our `rand` crate by adding as a dependency.
  - The trait `Rng` can be accessed using `rand::Rng`.
  - Keeping a crate's functionality in own scope/namespace helps prevent conflicts.
  - The name `Rng` in our current crate refers to the struct.

## 7.2 Defining Modules to Control Scope and Privacy

- _Modules_ lets us organize code within a crate into groups for readability and easy resus.

  - Can also control the privacy of the items
  - The library example helps us understand the _module tree_
  - We define a module using `mod` like _front_of_house_.
  - A module can also have `mod` children like _hosting_ and _serving_.

- Remember that either main.rs or lib.rs form the module at the root of the crate's module structure, the _module tree_.
- It helps to think of this as a file directory structure.

## 7.3 Paths for Referring to an Item in the Module Tree

- Absolute paths start from crate.
- Relative paths start from where it's called, in the same level in the current module.
- Both use `::`

- Choosing between using relative or absolute is going to be project based.

- Modules all have _privacy boundary_.
- The child can see the context outside but the outside can't see inside the child.
- Akin to the manager's office at the restaurant.
- This helps by not breaking the outer code if you can't the inside.

- **Exposing Paths with the pub Keyword**

  - The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default.
  - We expose by using `pub`.

  - You can access things if they are in the same module even if there's no `pub` keyword.
  - `eat_at_restaurant` can access `front_of_house` freely.

  ```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
  ```

- **Starting Relative Paths with super**

- Super is useful to access things outside of the current module but in the same crate.
- Basically, reach outside so we need to refer to the "parent".
- `fix_incorrect_order` can reach for `serve_order` outside of the module.
- The parent in this module tree is `crate`.

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

- **Making Structs and Enums Public**
- Structs have all fields _private_ by default.
  - We can set public field like `toast` with dot notation.
  - We can only set private fields with an associated function like `summer()`.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        // This is private. Need summer() to set.
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

- Enums have all variants _public_ by default.

```rust
mod back_of_house {
    pub enum Appetizer {
        // Both variants are public by default.
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## 7.4 Bringing Paths Into Scope with the use Keyword

- We can bring in `use` to shorten path names and call methods as local.
- Can utilize absolute or relative paths with `use` also.
- Note, that `use` also check privacy, just like any other path.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Bring absolute path here.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Now only have to refer to the trait.
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- **Creating Idiomatic use Paths**

- Writing out the full path is less clear because don't know if the function was locally defined.
- The idiomatic way is to have at least `hosting::add_to_waitlist();`.
- However, it's idiomatic to specify the full path for structs, enums, and other items.

```rust
// If using full path...
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // In a large code file, don't know if this was local or imported.
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// Use full paths for structs, enums, and others...
use std::collections::HashMap;

fn main() {
    // Common convention that we should know that HashMap is always imported.
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- However if you bring 2 of the same type of functions, you need 2 separate imports.
- Or you can give them aliases.

```rust
// Using separate imports:
use std::fmt;
use std::io;

// We know this comes from fmt module.
fn function1() -> fmt::Result {
    // --snip--
}

// We know this comes from io module.
fn function2() -> io::Result<()> {
    // --snip--
}
```

- **Providing New Names with the as Keyword**

- Can create 2 different functions or give them aliases:

```rust
// Using aliased imports:
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

// No conflict with name Result.
fn function2() -> IoResult<()> {
    // --snip--
}
```

- **Re-exporting Names with pub use**

- _Re-exporting_ is the process of exposing modules to external code
  - A dual move that imports the module and also exposes to external code.
  - Syntax is `pub use`.
- By default, new modules names we bring in are private.
  - Other external code could not use our module.
  - Now we can with `pub use`.
- The whole process lets us write 1 structure but expose different parts.
  - Helps organize the code for developers.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// `pub use` lets external code access this module.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- **Using External Packages**

- `std` is an external crate but it is shipped with the Rust language.
  - So we don't have to add it to `Cargo.toml` file.
  - We just add `use` to our file.
- `rand` was an external package we have to download from crates.io.
  - We had to import it in our `Cargo.toml` file.
  - Then we can use in a file:

```rust
// std crate already shipped with Rust language,
// only need to import into file.
use std::collections::HashMap;

// Brings in rand package with Rng trait...
use rand::Rng;

fn main() {
    // now can use thread_rng method.
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

- **Using Nested Paths to Clean Up Large use Lists**

- This helps saves multiple lines of imports.
- We can also write subpaths by using `self`.

```rust
// Multiple crates: from this...
use std::cmp::Ordering;
use std::io;

// to this.
use std::{cmp::Ordering, io};



// Same crate: from this...
use std::io;
use std::io::Write;

// to this.
use std::io::{self, Write};
```

- **The Glob Operator**

- Can bring in all the public items but not recommended.
  - Uses `*` operator like `use std::collections::*;`
  - Hard to tell which names are in scope.
  - Often used for testing or prelude pattern. More in Chapter 11.

## 7.5 Separating Modules into Different Files

- When we have multiple files, we can also import modules a certain way.
  - Basically the folder structure works like items in the path.
  - Can use `src/front_of_house.rs` and call with `mod front_of_house;`
  - Or can use `src/front_of_house/hosting.rs` system.
  - This lets us organize our code.
  - See the separating_modules for example.

# 8.0 Common Collections

- _Collections_ can contain multiple values unlike other data types.
  - This data is stored on the heap.
  - Data need not be known until compile time.
  - Can grow or shrink during program running.
  - 3 types: vector, string, hash map

## 8.1 Storing Lists of Values with Vectors

- **Creating a New Vector**

  - _vector_ is a collection of the same type of values stored in a contiguous block in memory.
  - `let v: Vec<i32> = Vec::new();` is how to create a new empty vector. Need to specify type since the vector is empty.
  - `let v = vec![1, 2, 3]` is using `vec!` macro to create a vector with data inside. Type is automatically inferred by Rust.
  - `Vec<T>` can hold any type and T is called a _generic_.

- **Updating a Vector**
  - You can push to an existing vector with `push` method.
  - We need `mut` in order to update the vector.
  - The numbers inside `v` are of type i32, inferred by Rust.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

- **Dropping a Vector Drops Its Elements**
  - If a vector is freed, all the elements get cleaned up too.

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here, along with elements.
```

- **Reading Elements of Vectors**

- ## Two ways to read a vector's content: using index or `get` method

  - Method 1: Index method uses `&` and `[]`.
    - Panics if index is out of range.
  - Method 2: Get method uses `.get(index)` with index being zero-based.
    - This returns an `Option<&T>` by default. Needs to be handled.
    - Does not panic when index out of range, returns `None` instead.

```rust
let v = vec![1, 2, 3, 4, 5];

// Method 1
let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

- Ownership and borrowing rules still apply:
  - If `&v[0]` references a mutable vector, any changes could possibly move the vector to another space in memory.
  - Remember that vectors are contiguously stored in memory, so if the data gets too big, all the bytes of the vector has to moved.
  - Consequently, you would be referencing a deallocated block.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}
```

- **Iterating over the Values in a Vector**
- We can loop through vectors with `for` loops in two ways:
  - In the second way, we have to dereference `i` with `*`.
  - The same reason above because a vector can move if the size doesn't fit the allocated memory block any longer.
  - So we have to say that `i` could be moved too.

```rust
// Looping through immutable vector.
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// Looping through mutable vector.
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

- **Using an Enum to Store Multiple Types**

- We can combine vectors with `enum` to make our vectors more flexible.
  - Vectors can only take 1 type of data but enums can have varied types.
  - This allows us to help Rust by declaring some memory allocation on the heap beforehand for the known types but also allow flexibility for the unknown types that come into the enum.
  - We know that there are 3 items of type `SpreadsheetCell` for the vector which follows the rules.
  - With the `enum`, we can restricts what types are allowed each the vector.
  - If we don't know all the types of the program, use `trait` object instead. More in chapter 17.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## 8.2 Storing UTF-8 Encoded Text with Strings

- Strings are complex in Rust compared to other languages.
- We look at creating, updating, and reading.
- Indexing a string is not straightforward.

- **What Is a String?**

- String literals are string slices `str` used with `&str` and are core to Rust language. Another term is the _borrowed_ type. This one is static at run time.
- `String` type is provided by Rust's standard library. Another term is the _owned_ type. This one is growable and mutable at run time.
- When Rustaceans talk about strings, they are talking about both string slices and `String` types are the same time.
- Both are UTF-8 encoded.

- There are other types but not covered here: `OsString`, `OsStr`, `CString`, `CStr`.
- These are owned and borrowed variants.

- **Creating a New String**

  - Use `String::new();` or `to_string();`

    ```rust
    // Method 1
    let mut s = String::new();
    let s = String::from("initial contents");

    // Method 2
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    ```

- These are all valid UTF-8 valid string values:

  ```rust
  let hello = String::from("السلام عليكم");
  let hello = String::from("Dobrý den");
  let hello = String::from("Hello");
  let hello = String::from("שָׁלוֹם");
  let hello = String::from("नमस्ते");
  let hello = String::from("こんにちは");
  let hello = String::from("안녕하세요");
  let hello = String::from("你好");
  let hello = String::from("Olá");
  let hello = String::from("Здравствуйте");
  let hello = String::from("Hola");
  ```

- **Updating a String**
- **Appending to a String with push_str and push**

  - Can use `push_str()` and `push()` to update strings.
  - `push_str()` takes a string slice and does NOT take ownership.
  - Notice `push_str` takes a string slice in code below. Otherwise would fail. You wouldn't have been able to println! if s2 transferred ownership to push_str.

    ```rust
    let mut s = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // => foobar

    // push() only takes 1 character as a parameter.
    let mut s = String::from("lo");
    s.push('l'); // => lol
    ```

- **Concatenation with the + Operator or the format! Macro**

  - Using `+` is tricky unless you truly understand it. Better to use `format!` macro.
  - s1 ownership was taken so we can't reuse.
  - Clues lie in the signature of the method `+` operator, or add method: `fn add(self, s: &str) -> String {`
  - Note that `add()` does not take ownership of s2, so that's still valid.

  - `+` works only with `&str` and `String`, but why does it work here?
  - `&s2` is a `&String` type, but there's _deref coercion_ at play here.
  - The compiler coerces `&String` into `&str` here. From `&s2` into `&s2[..]`. More in chapter 15.

  - What's truly happening here is `s1` was taken ownership by `+` and a copy of `s2` is appended. This is more efficient than copying both.

    ```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 ownership has been moved here and can no longer be used
    ```

- It can get crazy with multiple additions so use `format!` macro.

  - `format!` doesn't take ownership of any of the parameters.

    ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The hard way
    let s = s1 + "-" + &s2 + "-" + &s3;

    // The easy way
    let s = format!("{}-{}-{}", s1, s2, s3);
    ```

- **Indexing into Strings**

  - You can't index a string because some characters can span 2 bytes instead of 1.
  - However, Rust just doesn't handle indexing of strings at all.

    ```rust
    // Won't work, period!
    let s1 = String::from("hello");
    let h = s1[0];
    ```

- **Internal Representation**

  - A `String` is a wrapper over a `Vec<u8>`.

    ```rust
    // 4 bytes total
    let hello = String::from("Hola");

    // Actually 24 bytes, not 12 as you would think.
    let hello = String::from("Здравствуйте");

    // Trying to get the Cyrillic letter Ze (З) is not easy.
    // Index 0 is only half of the first character.
    // You need both 1st and 2nd bytes to represent Ze.
    let hello = "Здравствуйте";
    let answer = &hello[0]; // fails
    ```

- So Rust doesn't handle indexing at all!
- "To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process."

- **Bytes and Scalar Values and Grapheme Clusters! Oh My!**

  - These are the 3 ways to look at strings.
  - grapheme clusters are the closest thing to what we would call letters.
  - Indexes of strings should be O(1) operation but we can't guarantee that with how complicated strings are.

    ```rust
    // Hindi word: नमस्ते
    // Stored 18 bytes as vec<u8>: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // 4th and 6th are diacritics (don't make sense on own): ['न', 'म', 'स', '्', 'त', 'े']
    // When we mean letters with diacritics removed: ["न", "म", "स्", "ते"]
    ```

- **Slicing Strings**

  - We can use string slices to grab letters but it's dangerous.
  - It can crash your program if you're not careful.

    ```rust
    let hello = "Здравствуйте";

    // This works because each "letter" here is 2 bytes. => Зд
    let s = &hello[0..4];

    // Will error because only half of the first letter. => panic
    let t = &hello[0..1];
    ```

- **Methods for Iterating Over Strings**

  - Use `chars` or `bytes` method to iterate over elements.
  - `grapheme clusters` is not covered here.
  - Functionality is not provided in standard library.
  - Can check out crates.io.

    ```rust
    // chars method. This works if you're wanting characters.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes method. This works if you want raw byte.
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    ```

- **Strings Are Not So Simple**
  - Strings are harder in Rust but they prevent errors involving non-ASCII characters down the line.

## 8.3 Storing Keys with Associated Values in Hash Maps

- `HashMap<K, V>` stores keys and values
- Uses a _hashing function_ to determine location in memory.
- It has many names: hash, map, object, hash table, dictionary, associative array.
- Useful if you want to look up something quickly.

- **Creating a New Hash Map**

  - HashMap is not part of the standard library so much be imported.
  - Least commonly used than `vectors` and `String`, which are included in prelude.
  - Not much support from standard library, so no macro method to create.
  - Keys are of type `String` and values of type `i32`.
  - All keys and values must be homogenous types.

  ```rust
  fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
  }
  ```

  - We can also use vectors of tuple to create a Hash Map with `collect`.
  - More in chapter 13.

  ```rust
  fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
  }
  ```

- **Hash Maps and Ownership**

  - Types that implement `Copy` trait (i32) are copied onto hash map.
  - Owned types like `String` are moved and ownership taken.
  - `insert` in the code sample has moved the ownership into the hash map.

  ```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
  ```

- **Accessing Values in a Hash Map**

  - If you use `get` method, you will get an `Option<&V>`. So much be handled if you want the value.

  ```rust
  fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // => Some(&10)
  }
  ```

  - Iterating over key/value pair is like vector.
  - The print will be in arbitrary order, not guaranteed.

  ```rust
  fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
  }
  ```

- **Updating a Hash Map**

  - Many ways to interact with the hash map.
  - Can only ever be 1 key associated to 1 value.

- **Overwriting a Value**

  - Will overwrite with the latest value.

  ```rust
  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores); // => {"Blue": 25}
  ```

- **Only Inserting a Value If the Key Has No Value**

  - Use `entry` API to insert value. Uses Entry enum under the hood.
  - `or_insert` returns a mutable reference to value if key exists.
  - Otherwise, it will insert the new value.

  ```rust
  use std::collections::HashMap;

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  // Inserts new value 50 for yellow.
  scores.entry(String::from("Yellow")).or_insert(50);

  // Does NOT insert 50 because 10 already exists.
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores); // => {"Yellow": 50, "Blue": 10}
  ```

- **Updating a Value Based on the Old Value**

  - Inserts 0 if there's no entry in the hash map.
  - `count` interacts on the value returned.

  ```rust
  use std::collections::HashMap;

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      // returns &mut v
      let count = map.entry(word).or_insert(0);
      // Must dereference the &mut v with * before usage.
      *count += 1;
  }

  println!("{:?}", map); // => {"world": 2, "hello": 1, "wonderful": 1}
  ```

- **Hashing Functions**

  - Hash maps use `SipHash` by default to resist DoS attacks.
  - Tradeoff for speed for security. Not as fast hashing algorithm available.
  - If too slow, use a different hasher.
  - Hasher uses `BuildHasher` trait. More in chapter 10.
  - Many hashers on crates.io, don't have to implement own.

- **Chapter 8 Summary**
  - Lots of methods to use for vectors, strings, and hashmaps.
  - See standard library API documentation.
  - Now you can do exercises:
  1. Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
  2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
  3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

# 9.0 Error Handling

- Rust requires handling of errors before the code will compile.
- _recoverable_ and _unrecoverable_ errors.
- Recoverable is like selecting a wrong file and trying again.

  - Use `Result<T, E>`.

- Unrecoverable is like symptoms of bugs.
  - Use `panic!`.

## 9.1 Unrecoverable Errors with panic

- `panic!` execution will print a failure message, unwind and process the stack, and then quit.

- **Unwinding the Stack or Aborting in Response to a Panic**

  - Undwinding and cleanup is a lot of work, we can just _abort_ instead.
  - This leaves the file cleanup for the operating system.
  - Can use `panic = 'abort'` in `[profile]` section of Cargo.toml file.

  - Indicates error on 2nd line, 5th character and our personalized message.
  - Also cases where the error line points to the code that our code calls.

  ```rust
  fn main() {
    panic!("crash and burn");
  }

  // $ cargo run
  // Compiling panic v0.1.0 (file:///projects/panic)
  // Finished dev [unoptimized + debuginfo] target(s) in 0.25s
  // Running `target/debug/panic`
  // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
  // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```

- **Using a panic! Backtrace**

  - We try to get the 100th item. Rust will panic here.
  - In C, _buffer overread_ eads that block in memory even if it's not suppose to (security issue)
  - In Rust, it will stop execution and refuse to continue.

  - The last line tells use to run a _backtrace_ or a list of all the functions that have been called to get to this point.
  - To see this, the debug symbols must be enabled.
  - This is enabled by default with `cargo build` or `cargo run` without the --release flag.

  ```text
  fn main() {
      let v = vec![1, 2, 3];

      v[99];
  }

  // Error return.
  $ cargo run
  Compiling panic v0.1.0 (file:///projects/panic)
  Finished dev [unoptimized + debuginfo] target(s) in 0.27s
  Running `target/debug/panic`
  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  // Running a backtrace.
  $ RUST_BACKTRACE=1 cargo run
  thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
  stack backtrace:
    0: rust_begin_unwind
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
    1: core::panicking::panic_fmt
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
    2: core::panicking::panic_bounds_check
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
    3: <usize as core::slice::index::SliceIndex<[T]>>::index
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
    5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
    6: panic::main
              at ./src/main.rs:4
    7: core::ops::function::FnOnce::call_once
              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  ```

## 9.2 Recoverable Errors with Result

- Sometimes we just want to take another action if there's an error.
- We don't want to quit the operation completely.
- Like creating a new file, if the file doesn't exist.
- We can use the `Result` enum for this:

  - Where _T_ is the any type that returns with successfully Ok.
  - _E_ is the error returning with problem Err.

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

- We attempt to open a file.

  - We don't know the return type but the compiler will let us know.
  - We could have also looked in the standard library API documentation.

  - This example could have returned the file or the errors.
  - Could have been the file does not exist or not sufficient access permissions.

  ```rust
  // Listing 9-3: Opening a file
  use std::fs::File;

  fn main() {
      // Rust will return default results.
      // let f = File::open("hello.txt");

      // But we want to force a wrong return type.
      let f: u32 = File::open("hello.txt");

      // => expected type `u32`
      //     found enum `std::result::Result<File, std::io::Error>`
  }
  ```

  - Now, we'll handle the error with `match`.
  - The `Result` enum variants are brought in by the prelude ready to be used.

  ```rust
  // Listing 9-4: Using a match expression to handle the Result variants that might be returned
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt");

      let f = match f {
          // Return file if successful.
          Ok(file) => file,
          // Quit the program and return the default error.
          // => thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
          Err(error) => panic!("Problem opening the file: {:?}", error),
      };
  }
  ```

- **Matching on Different Errors**

  - Since we can output the errors, we possibly want to take action based on the error.
  - We want to create a new file if `NotFound` error.
  - That also needs a match check expression.
  - `error.kind()` automatically checks the kind on the `io::Error` return from the Err variant.

  - This code is verbose, we'll learn closures in Chapter 13.
  - `unwrap_or_else()` is one of these methods that cleans up nested match statements.

  ```rust
  // Listing 9-5: Handling different kinds of errors in different ways
  use std::fs::File;
  use std::io::ErrorKind;

  fn main() {
      let f = File::open("hello.txt");

      let f = match f {
          Ok(file) => file,
          // Return type of Err variant is `io::Error`.
          Err(error) => match error.kind() {
              ErrorKind::NotFound => match File::create("hello.txt") {
                  Ok(fc) => fc,
                  Err(e) => panic!("Problem creating the file: {:?}", e),
              },
              other_error => {
                  panic!("Problem opening the file: {:?}", other_error)
              }
          },
      };
  }

  // More advance, functionally same refactor
  use std::fs::File;
  use std::io::ErrorKind;

  fn main() {
      let f = File::open("hello.txt").unwrap_or_else(|error| {
          if error.kind() == ErrorKind::NotFound {
              File::create("hello.txt").unwrap_or_else(|error| {
                  panic!("Problem creating the file: {:?}", error);
              })
          } else {
              panic!("Problem opening the file: {:?}", error);
          }
      });
  }
  ```

- **Shortcuts for Panic on Error: unwrap and expect**

  - `unwrap()` is a shortcut method for match expressions.
  - It returns either the Ok with specified output or Err with `panic!`.
  - `expect()` is the same as unwrap but lets use choose the error message with `panic!`.
    - This can help us find our code error much faster in the source.

  ```rust
  //===Using unwrap()
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt").unwrap();
  }

  // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/libcore/result.rs:906:4

  //===Using Panic
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt").expect("Failed to open hello.txt");
  }

  // thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
  ```

- **Propagating Errors**

  - _propagating the error_ is returning the error so it can decide what to do.

  - Now we try to grab a username from the file.
  - Note the Result output type for both operations still follows the `Result<T,E>` pattern.
  - Common problem was why the reading the file failed?
    - Do we want to crash program, use a default username, or look elsewhere for the username?
    - Check out `?` operator.

  ```rust
  //===Listing 9-6: A function that returns errors to the calling code using match
  use std::fs::File;
  use std::io;
  use std::io::Read;

  fn read_username_from_file() -> Result<String, io::Error> {
      let f = File::open("hello.txt");

      // Opening the file.
      // Returns either the file or the error to f.
      let mut f = match f {
          Ok(file) => file,
          Err(e) => return Err(e),
      };

      // Declaring variable for outputs from reading contents of `f`.
      let mut s = String::new();

      // Reading the file.
      // If errors, what was the reason?
      // We will look at question mark operator.
      match f.read_to_string(&mut s) {
          Ok(_) => Ok(s),
          Err(e) => Err(e),
      }
  }
  ```

  - Same functionally as above.
  - The error value gets propagated to the calling code.
  - `?` is different from match is that it goes through `from` function.
    - It's derived from the `From` trait in the standard library.
    - When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.
    - This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically.
    - Chaining makes this process even shorter.
    - 9-9 is so common of an operation in that there's a method already built for reading a file into a string.

  ```rust
  //===Listing 9-7: A function that returns errors to the calling code using the ? operator
  use std::fs::File;
  use std::io;
  use std::io::Read;

  fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
  }

  //===Listing 9-8: Chaining method calls after the ? operator
  use std::fs::File;
  use std::io;
  use std::io::Read;

  fn read_username_from_file() -> Result<String, io::Error> {
      let mut s = String::new();

      File::open("hello.txt")?.read_to_string(&mut s)?;

      Ok(s)
  }

  //===Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
  use std::fs;
  use std::io;

  fn read_username_from_file() -> Result<String, io::Error> {
      fs::read_to_string("hello.txt")
  }
  ```

- **The ? Operator Can Be Used in Functions That Return Result**

  - We can't use the file operation in main() because it returns unit type.
  - But we can change that: 1) Add a return type 2) Using `match` combo with `Result<T,E>`
  - `Box<dyn Error>` is a special trait meaning any kind of error. More in chapter 17.

  ```rust
  //==Return with main()
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt")?;
  }
  // => cannot use the `?` operator in a function that returns `()`
  // => this function should return `Result` or `Option` to accept `?`
  // => help: the trait `Try` is not implemented for `()`

  //==Returning with Result type
  use std::error::Error;
  use std::fs::File;

  fn main() -> Result<(), Box<dyn Error>> {
      let f = File::open("hello.txt")?;

      Ok(())
  }
  ```

## 9.3 To panic or Not to panic

- When you invoke `panic!`, it's unrecoverable and you're making the decision already on behalf of your code.
  - There are exceptions that panicking is more appropriate.
- A better way to handle might be returning `Result` enum instead when defining a function that might fault.

  - You can then have the Err variant panic! here instead.

- **Examples, Prototype Code, and Tests**

  - Using `unwrap` or `expect` are handy during prototyping.
  - Signals that you should return to your code to handle the error more robustly.

- **Cases in Which You Have More Information Than the Compiler**

  - Sometimes have more information than the compiler, which you can then use `unwrap` acceptably.
  - We know that this IP address is valid so it's impossible to error.
  - The compiler doesn't know this so we should handle the Err anyways for it.
  - In cases of user-input, we definitely have to error check.

  ```rust
  use std::net::IpAddr;

  let home: IpAddr = "127.0.0.1".parse().unwrap();
  ```

- **Guidelines for Error Handling**

  - If you're code goes into a _bad state_, it's advisable to panic.
  - _bad state_ is when you have the structure breaks due to invalid values, contradictory values, or missing values.
  - It should also have one of the following:

  1. This _bad state_ not normally expected to occasionally happen.
  2. Code needs to rely on output that's not a bad state.
  3. Not a good way to encode this info in the types you use.

  - So examples are if someone passes in bad values or your call to another external code that returns an invalid state that you can't fix. These are situations appropriate to panic.

  - A good ground rule is to check for valid values and then panic.
  - This closes code vulnerabilities and transfer responsibility to the calling programmers.

  - _contracts_ are behaviors that occur only if the inputs meet a requirement.
  - When contracts are violated, it's logical the code doesn't/shouldn't handle it.
  - An example is when accessing an out-of-bound index.
  - Issues like this should be explained in the API documentation.

  - Another common practice is to only allow a type rather than using `Option`.
  - This saves us from having to handle both `Some` and `None`.
  - Using `u32` can also save us from the value never being negative.

- **Creating Custom Types for Validation**

  - In the guessing game, we only checked for positive numbers.

    - This works but we want to be more specific.
    - We want to check between specifically within the rang of 1-100.

  - Let's say we have a loop that needs to check if the number is i32.
  - Instead of checking this everytime, let's just make a struct instead.
  - Anytime a new `Guess` is made, the value is check for validity.
  - Since struct members are private, we know it's always valid.
  - The only time there's a write was during instantiation, it's never changed any any other point in time.
  - `value` is a getter which allows only retrieval.

  ```rust
  //==Inefficient checking
    loop {
      // --snip--

      let guess: i32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      };

      if guess < 1 || guess > 100 {
          println!("The secret number will be between 1 and 100.");
          continue;
      }

      match guess.cmp(&secret_number) {
          // --snip--
  }

  //==Efficient type checking
  pub struct Guess {
    value: i32,
  }

  impl Guess {
      // Checks value when `Guess::new` is called.
      pub fn new(value: i32) -> Guess {
          if value < 1 || value > 100 {
              panic!("Guess value must be between 1 and 100, got {}.", value);
          }

          Guess { value }
      }

      // A getter that only retrieves, zero write methods.
      pub fn value(&self) -> i32 {
          self.value
      }
  }
  ```
