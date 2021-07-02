// We use `mut` if we want a variable to be mutable.
// If it doesn't run try reinstall xcode.
// https://stackoverflow.com/questions/28124221/error-linking-with-cc-failed-exit-code-1
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

// Leads to error: error[E0384]: cannot assign twice to immutable variable `x`
// By default you can't just change the variable value.
// fn main() {
//   let x = 5;
//   println!("The value of x is: {}", x);
//   x = 6;
//   println!("The value of x is: {}", x);
// }
