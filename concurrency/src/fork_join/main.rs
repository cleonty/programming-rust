#![allow(unused)]

use std::sync::Arc;

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

struct GigabyteMap {
  
}

fn process_files(filenames: Vec<String>, glossary: &GigabyteMap) -> std::io::Result<()> {
  for document in filenames {
    let text = load(&document)?; // read source file
    let results = process(text); // compute statistics
    save(&document, results)?; // write output file
  }
  Ok(())
}

use std::thread::spawn;

fn process_files_in_parallel(filenames: Vec<String>, glossary: Arc<GigabyteMap>) -> std::io::Result<()> {
  // Divide the work into several chunks.
  const NTHREADS: usize = 8;
  let worklists = filenames.chunks(NTHREADS).collect::<Vec<_>>();
  // Fork: Spawn a thread to handle each chunk.
  let mut thread_handles = vec![];
  for worklist in worklists {
    let copy = worklist.to_vec();
    let glossary_for_child = glossary.clone();
    thread_handles.push(spawn(move || process_files(copy, &glossary_for_child)));
  }
  // Join: Wait for all threads to finish.
  for handle in thread_handles {
    handle.join().unwrap()?;
  }
  Ok(())
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
  let giga_map = GigabyteMap{};
  let glossary = Arc::new(giga_map);
  
  process_files_in_parallel(filenames, glossary)
  // process_files(filenames)
}
