#![allow(dead_code)]
#![allow(unused_variables)]

fn triangle1(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

fn triangle2(n: i32) -> i32 {
    (1..n + 1).fold(0, |sum, item| sum + item)
}

#[test]
fn test_triangle() {
    let n = 10;
    let t1 = triangle1(n);
    let t2 = triangle2(n);
    assert_eq!(t1, t2);
}

#[test]
fn test_for() {
    println!("There's");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{}", element);
    }
}

#[test]
fn test_iter() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);
}

#[test]
fn test_iter_path() {
    use std::ffi::OsStr;
    use std::path::Path;
    let path = Path::new("C:\\Users\\JimB\\Downloads\\Fedora.iso");
    let mut iterator = path.iter();
    for element in path {
        println!("element {:?}", element);
    }
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("\\")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));
}

#[test]
fn test_iter_btreeset() {
    // You should usually use HashSet, but its iteration order is
    // nondeterministic, so BTreeSet works better in examples.
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());
    for song in &favorites {
        println!("song {}", song);
    }
    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);
}

#[test]
fn test_dump() {
    use std::fmt::Debug;
    fn dump<T, U>(t: T)
    where
        T: IntoIterator<Item = U>,
        U: Debug,
    {
        for u in t {
            println!("{:?}", u);
        }
    }
}

#[test]
fn test_drain() {
    use std::iter::FromIterator;
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}

#[test]
fn test_map() {
    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|a| *a != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}

#[test]
fn test_filter_map() {
    use std::str::FromStr;
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }
}

#[test]
fn test_flat_map() {
    use std::collections::HashMap;
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}

#[test]
fn test_scan() {
    let iter = (0..10).scan(0, |sum, item| {
        *sum += item;
        if *sum > 10 {
            None
        } else {
            Some(item * item)
        }
    });
    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);
}

#[test]
fn test_take() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";
    let headers_iter = message.lines().take_while(|l| !l.is_empty());
    assert_eq!(headers_iter.collect::<Vec<_>>().len(), 2);
}
#[test]
fn test_skip() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";
    let body_iter = message.lines().skip_while(|l| !l.is_empty()).skip(1);
    assert_eq!(body_iter.collect::<Vec<_>>().len(), 2);
}

#[test]
fn test_peek() {
    use std::iter::Peekable;
    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n,
            }
            tokens.next();
        }
    }
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // Look, `parse_number` didn't consume the comma! So we will.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);
}

#[test]
fn test_fuse() {
    struct Flaky(bool);
    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));
    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

#[test]
fn test_rev() {
    use std::iter::DoubleEndedIterator;
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_inspect() {
    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");
}

#[test]
fn test_chain() {
    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);
    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);
}

#[test]
fn test_zip() {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
}

#[test]
fn test_zip2() {
    use std::iter::repeat;
    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice")
        ]
    );
}

#[test]
fn test_by_ref() {
    let message = "To: jimb\r\n\
                   From: id\r\n\
                   \r\n\
                   Oooooh, donuts!!\r\n";
    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }
}

#[test]
fn test_clone() {
    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));
}

#[test]
fn test_cycle() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
}

#[test]
fn test_cycle2() {
    use std::iter::{once, repeat};
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    for line in fizz_buzz {
        println!("{}", line);
    }
}

#[test]
fn test_sum_and_product() {
    fn triangle(n: u64) -> u64 {
        (1..n + 1).sum()
    }
    assert_eq!(triangle(20), 210);
    fn factorial(n: u64) -> u64 {
        (1..n + 1).product()
    }
    assert_eq!(factorial(20), 2432902008176640000);
}

#[test]
fn test_min_max() {
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
}

#[test]
fn test_max_by_and_min_by() {
    use std::cmp::{Ordering, PartialOrd};
    // Compare two f64 values. Panic if given a NaN.
    fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
    assert_eq!(numbers.iter().min_by(cmp), Some(&1.0));
}
#[test]
#[should_panic]
fn test_max_by_and_min_by_panic() {
    use std::cmp::{Ordering, PartialOrd};
    // Compare two f64 values. Panic if given a NaN.
    fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }
    let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0)); // panics
}

#[test]
fn test_max_and_min_by_key() {
    use std::collections::HashMap;
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);
    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"Portland", &583_776))
    );
    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Greenhorn", &2))
    );
}

#[test]
fn test_comparing_strings() {
    let packed = "Helen of Troy";
    let spaced = "Helen    of     Troy";
    let obscure = "Helen of Sandusky"; // nice person, just not famous
    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    // This is true because ' ' < 'o'.
    assert!(spaced < obscure);
    // This is true because 'Troy' > 'Sandusky'.
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}

#[test]
fn test_all_any() {
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));
}

#[test]
fn test_position() {
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);
    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b's'), Some(5));
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));
}

#[test]
fn test_fold() {
    let a = [5, 6, 7, 8, 9, 10];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6); // count
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45); // sum
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200); // product
    assert_eq!(
        a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m, i)),
        10
    ); // max
    assert_eq!(
        a.iter().fold(i32::max_value(), |m, &i| std::cmp::min(m, i)),
        5
    ); // min

    let a = [
        "Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs",
    ];
    let pangram = a.iter().fold(String::new(), |mut s, &w| {
        s.push_str(w);
        s
    });
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs");
}

#[test]
fn test_nth() {
    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(0), Some(36));
    assert_eq!(squares.nth(6), None);
}

#[test]
fn test_last() {
    let squares = (0..11).map(|i| i * i);
    assert_eq!(squares.last(), Some(100));
}

#[test]
fn test_collect() {
    let args: Vec<String> = std::env::args().collect();
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};
    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();
    // Collecting a map requires (key, value) pairs, so for this example,
    // zip the sequence of strings with a sequence of integers.
    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
}

#[test]
fn test_extend() {
    let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
    v.extend(&[31, 57, 99, 163]);
    assert_eq!(v, &[1, 2, 4, 8, 16, 31, 57, 99, 163]);
}

#[test]
fn test_partition() {
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    // Amazing fact: the name of a living thing always starts with an
    // odd-numbered letter.
    let (living, nonliving): (Vec<&str>, _) =
        things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);
    assert_eq!(living, vec!["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, vec!["doorknob", "noodle"]);
}

struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T>{
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

use self::BinaryTree::*;

struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T> TreeIter<'a, T>  where T: 'a {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T>  {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) -> BinaryTree<T> {
    NonEmpty(Box::new(TreeNode {
        left,
        element,
        right,
    }))
}

#[test]
fn test_tree_iter() {
    let subtree_l = make_node(Empty, "mecha", Empty);
    let subtree_rl = make_node(Empty, "droid", Empty);
    let subtree_r = make_node(subtree_rl, "robot", Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);
    // Iterate over it.
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);
    assert_eq!(
        tree.iter()
            .map(|name| format!("mega-{}", name))
            .collect::<Vec<_>>(),
        vec!["mega-mecha", "mega-Jaeger", "mega-droid", "mega-robot"]
    );
}

#[test]
fn test_i32_range_iterator() {
    let mut pi = 0.0;
    let mut numerator = 1.0;
    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);
    assert_eq!(pi as f32, std::f32::consts::PI);
}

fn main() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";
    let body_iter = message.lines().skip_while(|l| !l.is_empty()).skip(1);
    for line in body_iter {
        println!("line: '{}'", line)
    }
}

// CHAPTER 15 Iterators, Implementing Your Own Iterators 354
