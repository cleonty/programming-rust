use std::process::{Command, Stdio};
use std::io::Write;

fn main() -> std::io::Result<()> {
  let mut child = Command::new("ls")
    .arg("-al")
    .stdin(Stdio::piped())
    .spawn()?;
  let mut to_child = child.stdin.take().unwrap();
  let my_words = vec!["hello", "world"];
  for word in my_words {
    writeln!(to_child, "{}", word)?;
  }
  drop(to_child);
  child.wait()?;
  Ok(())
}
