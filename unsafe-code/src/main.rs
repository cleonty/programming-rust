#![allow(dead_code)]

#[test]
fn test_raw_pointers() {
  let mut x = 10;
  let ptr_x = &mut x as *mut i32;

  let y = Box::new(20);
  let ptr_y = &*y as *const i32;

  unsafe {
    *ptr_x += *ptr_y;
  }
  assert_eq!(x, 30)
}

#[test]
fn test_null() {
  fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
      None => std::ptr::null(),
      Some(r) => r as *const T,
    }
  }
  assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
  assert_eq!(option_to_raw::<i32>(None), std::ptr::null());
}

#[test]
fn test_distance() {
  fn distance<T>(left: *const T, right: *const T) -> isize {
    (left as isize - right as isize) / std::mem::size_of::<T>() as isize
  }
  let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
  let first = &trucks[0];
  let last = &trucks[2];
  assert_eq!(distance(last, first), 2);
  assert_eq!(distance(first, last), -2);
}

mod ref_with_flag;

#[test]
fn test_ref_with_flag() {
  let vec = vec![10, 20, 30];
  let flagged = ref_with_flag::ref_with_flag::RefWithFlag::new(&vec, true);
  assert_eq!(flagged.get_ref()[1], 20);
  assert_eq!(flagged.get_flag(), true);
}

#[test]
fn test_alignment() {
  // Fat pointers to slices carry their referent's length.
  let slice: &[i32] = &[1, 3, 9, 27, 81];
  assert_eq!(std::mem::size_of_val(slice), 20);
  let text: &str = "alligator";
  assert_eq!(std::mem::size_of_val(text), 9);
  use std::fmt::Display;
  let unremarkable: &dyn Display = &193_u8;
  let remarkable: &dyn Display = &0.0072973525664;
  // These return the size/alignment of the value the
  // trait object points to, not those of the trait object
  // itself. This information comes from the vtable the
  // trait object refers to.
  assert_eq!(std::mem::size_of_val(unremarkable), 1);
  assert_eq!(std::mem::align_of_val(remarkable), 8);
}

mod gap;

#[test]
fn test_gap() {
  use gap::gap::GapBuffer;
  let mut buf = GapBuffer::new();
  buf.insert_iter("Lord of the Rings".chars());
  buf.set_position(12);
}

use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
  fn strlen(s: *const c_char) -> usize;
  static environ: *mut *mut c_char;
}

#[test]
fn test_strlen() {
  use std::ffi::CString;
  let rust_str = "I'll be back";
  let null_terminated = CString::new(rust_str).unwrap();
  unsafe {
    assert_eq!(strlen(null_terminated.as_ptr()), 12);
  }
}

#[test]
fn test_print_env() {
}

fn main() {
}

// Foreign Functions: A Safe Interface to libgit2 on page 572
