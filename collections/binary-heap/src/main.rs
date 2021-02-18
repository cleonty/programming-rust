use std::collections::BinaryHeap;

#[test]
fn test_heap() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));
    while let Some(number) = heap.pop() {
        println!("{}", number);
    }
}

fn main() {
    println!("Hello, world!");
}
