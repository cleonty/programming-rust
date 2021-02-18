fn main() {
    use std::collections::LinkedList;
    let mut l = LinkedList::new();
    l.push_back(4);
    l.push_back(5);
    l.push_front(3);
    l.push_front(2);
    l.push_front(1);
    println!("{:?}", l);
}
