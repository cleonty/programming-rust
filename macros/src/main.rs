#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![recursion_limit = "256"]
#[macro_use] mod macros;

use std::collections::HashMap;

macro_rules! my_assert_eq {
    ($left: expr, $right: expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion failed: `(left == right)` \
                    (left: `{:?}`, right: `{:?}`))",
                        left_val, right_val
                    )
                }
            }
        }
    }};
}

#[test]
fn test_my_assert_eq() {
    my_assert_eq!(1, 1);
}

#[test]
#[should_panic]
fn test_my_assert_eq_panic() {
    my_assert_eq!(1, 2);
}

macro_rules! my_vec {
    ($elem:expr ; $n:expr) => {
    ::std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
    <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ),+ ,) => {
    vec![ $( $x ),* ]
    };
}

#[test]
fn test_my_vec() {
    let buffer = my_vec![0_u8; 1000];
    // A list of values, separated by commas
    let numbers = my_vec!["udon", "ramen", "soba"];
}

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);


fn main() {
    let numbers = my_vec!["udon", "ramen", "soba"];
    println!(stringify!(numbers));
    let width = 4.0;
    let desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });
}

// Macros, Scoping and Hygiene continuing on page 518, not from beginning
