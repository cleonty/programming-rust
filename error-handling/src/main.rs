#![allow(dead_code)]
fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

#[test]
fn test_pirate_share() {
    pirate_share(1000, 1);
}

use std::error::Error;
use std::io::{stderr, Write};

#[allow(dead_code)]
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(cause) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", err);
        err = cause;
    }
}

use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MyError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait.
impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[allow(dead_code)]
fn return_error() -> Result<(), MyError> {
    return Err(MyError {
        message: "expected not error".to_string(),
        line: 1,
        column: 5,
    });
}

#[test]
fn test_my_error() {
    match return_error() {
        Ok(_) => println!("ok"),
        Err(e) => println!("error {:?} {}", e, e)
    }
}

fn main() {
    let mut x = 5;
    x += 1;
    println!("{}", x);
}
