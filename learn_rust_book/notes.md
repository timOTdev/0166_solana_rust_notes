# The Rust Programming Language

- [Learn Rust Book](https://www.rust-lang.org/learn)

# Foreword

- Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way.
- Programmers who are already working with low-level code can use Rust to raise their ambitions.
- It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write.

# Introduction

- Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.
- In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs.
- *Cargo* is the included dependency manager and build tool.
- *Rustfmt* is the coding style.
- *Rust Language Server* for IDE integration.
- Big companies use Rust in production for command line tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even major parts of the Firefox web browser.
- Assumes you've written code in another language, not complete beginner to coding.
- Later chapters build on concepts in earlier chapters, and earlier chapters might not delve into details on a topic.
- Concept chapters and project chapters
- Appendices have reference-like materials

# 1.0 Getting Started

## 1.1 Installation

- I installed for windows.
- Commands in this book work in both *cmd.exe* and PowerShell.
- `rustup update`  - Updates rust to latest version
- `rustup self uninstall`  - Uninstalls rust
- `rustc --version` - Check Rust version.
- `rustup doc` - Load docs in browser and read offline.

## 1.2 Hello, World!

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

## 1.3 Hello, Cargo!

- Cargo is a build system and package manager that builds code, downloads libaries/dependencies, and builds them together.
- Cargo has the same commands for all OS
- Cargo expects your source files to live inside the src directory.
- The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
- `cargo --version` - Checks if cargo is installed.
- `cargo new hello_cargo` - Creates new cargo app like create-react-app. Will not generate Git files if an existing Git repository.
    - `cargo new --vcs=git` will generate Git files in an existing Git Repository.
- `Cargo.toml` is like package.json. Stands for Tom's Obvious, Minimal Language)
    - Packages are called *crates* in Cargo. Like *packages* for NPM.

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
      1. Are *always* immutable. You can't use `mut` with constants.
      2. Can be declared in any scope, including global.
      3. Set by constant expression, not result of a function call or computed values.
    - Convention to use all caps. IE `const MAX_POINTS: u32 = 100_000`
    - Constants last the lifetime for the whole of the program.
    - Scoped to where declared.

- Shadowing
    - When values carry over in subsequent declarations.
    - We are just transforming the values but we can't reassign the variable name without using `let` keyword.
    - Also allows changing of the type of the variable because we are *effectively creating a new variable with every new shadow.*

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

- Rust is a *statically typed language,* we must know types of all variables at compile time.
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
    - Numbers over the bit settings will cause *integer overflow* and *panic* at runtime (program exits with an error.
    - During —release flag, rust does *not* check for integer overflow that causes panic.

    - *Complement Wrapping* will circle back to start at the minimum value of the range.
    - There's 4 types to handle this overflow.
      1. wrapping_* wraps in all modes IE `wrapping_add`
      2. checked_* - Returns None value if overflow
      3. overflowing_* - Returns value and Boolean indicating overflow
      4. saturating_* - saturate with the minimum or maximum value.

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
      - Most other languages have mutable array lengths, *not* Rust.
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
