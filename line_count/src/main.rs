use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    println!("line count is {}", stdin.lock().lines().count());
}
