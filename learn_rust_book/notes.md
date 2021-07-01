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
