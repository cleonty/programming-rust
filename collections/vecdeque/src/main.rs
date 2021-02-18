fn main() {
    use std::collections::VecDeque;
    let mut v = VecDeque::from(vec![1, 2, 3, 4]);
    v.push_back(5);
    v.push_front(0);
    println!("{:?}", v);
}

//Collections, VecDeque<T> on page 374
