fn main() {
    println!("Hello, world!");
}

#[test]
fn test_use() {
    use std::mem;
    let mut s1 = 2;
    let mut s2 = 3;
    if s1 > s2 {
        mem::swap(&mut s1, &mut s2);
    }
}