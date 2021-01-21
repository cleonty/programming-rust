#![allow(dead_code)]
#![allow(unused_variables)]
#[warn(bare_trait_objects)]
#[derive(Debug)]
struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

use std::ops::Drop;

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

#[test]
fn test_drop_appellation() {
    let mut _a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };
    println!("before assignment");
    _a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at end of block");
}

#[test]
fn test_drop_appellation2() {
    let _p;
    {
        let q = Appellation {
            name: "Cardamine hirsuta".to_string(),
            nicknames: vec!["shotweed".to_string(), "bittercress".to_string()],
        };
        if 1 > 2 {
            _p = q;
        }
    }
    println!("Sproing! What was that?");
}

use std::fmt::Display;

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}

#[test]
fn test_rc_box() {
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };
    use std::fmt::Display;
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    display(&boxed_lunch);
}

struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,
    /// The index of the "current" element in `elements`. A `Selector`
    /// behaves like a pointer to the current element.
    current: usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}

#[test]
fn test_selector() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);
    show_it_generic(&*s);
}

#[test]
fn test_partition() {
    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & n - 1 == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);
}

#[test]
fn test_partition_2() {
    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");
}

use std::net::Ipv4Addr;

fn ping<A>(address: A) -> std::io::Result<bool>
where
    A: Into<Ipv4Addr>,
{
    let ipv4_address = address.into();
    Ok(true)
}

#[test]
fn test_ping() {
    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141))); // pass an Ipv4Addr
    println!("{:?}", ping([66, 146, 219, 98])); // pass a [u8; 4]
    println!("{:?}", ping(0xd076eb94_u32)); // pass a u32
    let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
    let addr2 = Ipv4Addr::from(0xd076eb94_u32);
}

use std::borrow::Cow;
use std::path::PathBuf;
fn describe(error: i32) -> Cow<'static, str> {
    match error {
        1 => "out of memory".into(),
        2 => "stack overflow".into(),
        3 => "machine on fire".into(),
        4 => "machine bewildered".into(),
        x => format!("file not found: {}", x).into(),
    }
}

#[test]
fn test_describe() {
    println!("Disaster has struck: {}", describe(55));
    let mut log: Vec<String> = Vec::new();
    log.push(describe(33).into_owned());
}

fn main() {}

// From and Into, page 297
