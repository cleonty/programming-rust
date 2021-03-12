#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, ErrorKind, Read, Write};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;
pub fn copy_clone<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

fn grep(target: &str) -> io::Result<()> {
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn grep2<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main()-> std::io::Result<()> {
    println!("Hello, world!");
    let target = "line";
    /*
    let stdin = io::stdin();
    grep2(&target, stdin.lock()).unwrap(); // ok
    */
    let cur_dir = std::env::current_dir()?;
    println!("{}", cur_dir.display());
    let file = "./src/main.rs";
    let f = File::open(file)?;
    grep2(&target, BufReader::new(f))?; // also ok
    Ok(())
}

// Input and Output, Files and Directories on page 445
