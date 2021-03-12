#![allow(unused)]

fn open_options() -> std::io::Result<()> {
  use std::fs::OpenOptions;
  let log = OpenOptions::new().append(true).open("server.log")?;
  let file = OpenOptions::new().write(true).create_new(true).open("new_file.txt")?;
  Ok(())
}

fn main() -> std::io::Result<()> {
  open_options()?;
  Ok(())
}
