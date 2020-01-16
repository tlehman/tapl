// 2020-01-15: While reading TaPL and working through the OCaml example code,
// I realized what I needed: a RePL (read-eval-print loop)

use std::io;

fn main() {
  let mut buffer = String::new();
  println!("Untyped λ-calculus REPL");
  while !buffer.starts_with("exit") {
    buffer = String::new();
    print!("λ-calculus: ");
    match io::stdin().read_line(&mut buffer) {
      Ok(n) => {
        println!("you typed {} bytes: {}", n, buffer);
      }
      Err(error) => {
        println!("error: {}", error);
      }
    }
  }
}