# rust_starter

- hello_world is initial rust app
- hello_cargo is working with build system and package manager

## Starting Tips

- Git files removed from hello_cargo for commit purposes.
- Normally, `cargo new hello_cargo` will generate src folder, Cargo.toml, .gitignore, and hidden .git directory

## Useful Commands

- Quick reference

```rust
// Terminal commands
$ rustfmt main.rs // To run style formatting.
$ rustc main.rs // To compile to exe.
$ ./main // To run the executable. Both \ and / worked for me in Windows 10.

$ cargo --version // Checks if cargo is installed
$ cargo new hello_cargo // Creates new cargo app
$ cargo build // Compiles cargo project to target/debug folder
$ ./target/debug/hello_cargo // Also works for me .\target\debug\hello_cargo.exe on Windows
$ cargo run // One command and combined cargo build + execute commands
$ cargo check // Checks code compile process without creating executable
$ cargo build --release // Compiles cargo project to target/release folder
```

## Resources

- Learning from [Rust Book](https://www.rust-lang.org/learn)
- Taking notes on [my notion page](https://www.notion.so/How-Rusty-Is-Your-Rust-Lang-fe1988349f1246598fbd5d653f301cc7)
