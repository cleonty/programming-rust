#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::prelude::*; // for `Read::read_to_string`
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread::spawn;
use std::thread::JoinHandle;

fn start_file_reader_thread(
  documents: Vec<PathBuf>,
) -> (Receiver<String>, JoinHandle<std::io::Result<()>>) {
  let (sender, receiver) = channel();
  let handle: std::thread::JoinHandle<std::result::Result<(), std::io::Error>> = spawn(move || {
    for filename in documents {
      let mut f = File::open(filename)?;
      let mut text = String::new();
      f.read_to_string(&mut text)?;
      if sender.send(text).is_err() {
        break;
      }
    }
    Ok(())
  });
  (receiver, handle)
}
struct InMemoryIndex {
  
}

impl InMemoryIndex {
  pub fn from_single_document(document_id: usize, text: String) -> InMemoryIndex {
    InMemoryIndex{}
  }
}

fn start_file_indexing_thread(
  texts: Receiver<String>,
) -> (Receiver<InMemoryIndex>, JoinHandle<()>) {
  let (sender, receiver) = channel();
  let handle = spawn(move || {
    for (doc_id, text) in texts.into_iter().enumerate() {
      let index = InMemoryIndex::from_single_document(doc_id, text);
      if sender.send(index).is_err() {
        break;
      }
    }
  });
  (receiver, handle)
}

fn main() {
  println!("Hello, world!");
}
