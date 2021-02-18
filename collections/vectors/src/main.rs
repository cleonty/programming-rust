#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#[test]
fn test_create_vec() {
    let mut numbers: Vec<i32> = vec![];
    let words = vec!["step", "on", "no", "pets"];
    let mut buffer = vec![0u8; 1024];
}

#[test]
fn test_access() {
    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);
    assert_eq!(slice.get(1), Some(&1));
    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap(); // type of last: &mut i32
        assert_eq!(*last, 3);
        *last = 100;
        assert_eq!(last, &100)
    }
    assert_eq!(slice, [0, 1, 2, 100]);
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_dedup() {
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");
}

#[test]
fn test_dedup2() {
    use std::collections::HashSet;
    let mut byte_vec = b"Misssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");
}

#[test]
fn test_concat_and_join() {
    assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(
        [[1, 2], [3, 4], [5, 6]].join(&0),
        vec![1, 2, 0, 3, 4, 0, 5, 6]
    );
}

#[test]
fn test_slice() {
    let v = vec![0, 1, 2, 3, 4];
    let a = &v[0];
    let b = &v[1];

    let mid = v.len() / 2;
    let front_half = &v[..mid];
    let back_half = &v[mid..];
}

#[test]
fn test_windows() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let diff = v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    assert_eq!(diff, [1, 1, 1, 1, 1]);
}

#[test]
fn test_retain() {
    let mut my_vec = vec![1, 3, 5, 7, 9];
    my_vec.retain(|&val| val <= 4);
}

fn main() {
    println!("Hello, world!");
}

// Collections, Vec<T>, Splitting on page 368
