#![allow(unused)]

extern crate rayon;
use rayon::prelude::*;

fn load(doc: &String) -> std::io::Result<String> {
  std::thread::sleep(std::time::Duration::from_secs(3));
  Ok("abc".to_string())
}

fn process(text: String) -> String {
  std::thread::sleep(std::time::Duration::from_secs(1));
  text
}

fn save(doc: &String, results: String) -> std::io::Result<()> {
  std::thread::sleep(std::time::Duration::from_secs(1));
  Ok(())
}

struct GigabyteMap {}

fn process_file(document: &String, glossary: &GigabyteMap) -> std::io::Result<()> {
  println!("process file begins");
  let text = load(&document)?; // read source file
  let results = process(text); // compute statistics
  save(&document, results)?; // write output file
  println!("process file ends");
  Ok(())
}

use std::thread::spawn;

fn process_files_in_parallel(
  filenames: Vec<String>,
  glossary: &GigabyteMap,
) -> std::io::Result<()> {
  // Divide the work into several chunks.
  filenames
    .par_iter()
    .map(|filename| process_file(filename, glossary))
    .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
    .unwrap_or(Ok(()))
}
fn main() -> std::io::Result<()> {
  let filenames = vec![
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
    "abc".to_string(),
  ];
  let giga_map = GigabyteMap {};
  let glossary = &giga_map;
  process_files_in_parallel(filenames, glossary)
  // process_files(filenames)
}
