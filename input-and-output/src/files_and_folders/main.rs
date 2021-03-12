use std::fs;
use std::io;
use std::path::Path;

#[test]
fn test_parent() {
  assert_eq!(
    Path::new("/home/fwolfe/program.txt").parent(),
    Some(Path::new("/home/fwolfe"))
  );
}

#[test]
fn test_filename() {
  use std::ffi::OsStr;
  assert_eq!(
    Path::new("/home/fwolfe/program.txt").file_name(),
    Some(OsStr::new("program.txt"))
  );
}

#[test]
fn test_join() {
  let path1 = Path::new("/usr/share/dict");
  assert_eq!(path1.join("words"), Path::new("/usr/share/dict/words"));
}

#[test]
fn test_filesystem() {
  let path = Path::new("./a/b/c");
  std::fs::create_dir_all(path).unwrap();
  let metadata = path.metadata().unwrap();
  assert_eq!(metadata.is_dir(), true);
  std::fs::remove_dir_all(path.parent().unwrap().parent().unwrap()).unwrap();
  std::fs::metadata(path).unwrap_err();
}

fn my_read_dir() -> std::io::Result<()> {
  let path = Path::new("./a/b/c");
  std::fs::create_dir_all(path)?;
  let top = path.parent().unwrap().parent().unwrap();
  for entry_result in top.read_dir()? {
    let entry = entry_result?;
    println!("{}", entry.file_name().to_string_lossy());
  }
  std::fs::remove_dir_all(top)?;
  Ok(())
}

/// Copy the existing directory `src` to the target path `dst`.
pub fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
  if !dst.is_dir() {
    fs::create_dir(dst)?;
  }
  for entry_result in src.read_dir()? {
    let entry = entry_result?;
    let file_type = entry.file_type()?;
    copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
  }
  Ok(())
}

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q) -> std::io::Result<()> {
  Err(io::Error::new(
    io::ErrorKind::Other,
    format!("can't copy symbolic link: {}", src.as_ref().display()),
  ))
}

/// Copy whatever is at `src` to the target path `dst`.
fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
  if src_type.is_file() {
    fs::copy(src, dst)?;
  } else if src_type.is_dir() {
    copy_dir_to(src, dst)?;
  } else if src_type.is_symlink() {
    let target = src.read_link()?;
    symlink(target, dst)?;
  } else {
    return Err(io::Error::new(
      io::ErrorKind::Other,
      format!("don't know how to copy: {}", src.display()),
    ));
  }
  Ok(())
}
#[allow(unused_variables)]
fn main() -> std::io::Result<()> {
  my_read_dir()?;
  Ok(())
}

// Platform-Specific Features on page 451
