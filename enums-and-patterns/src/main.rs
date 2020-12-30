pub enum MyOrdering {
    Less,
    Equal,
    Greater,
}

use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::HashMap;

pub fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

pub enum Pet {
    Orca,
    Giraffe,
    Elephant,
}

use self::Pet::*;

pub enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[test]
fn test_enum_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
    assert_eq!(HttpStatus::Ok as i32, 200);
}

pub fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

#[test]
fn test_plural_and_singular() {
    let minutes = TimeUnit::Minutes.plural();
    let minute = TimeUnit::Minutes.singular();
    assert_eq!(minutes, "minutes");
    assert_eq!(minute, "minute");
}

/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[test]
fn test_rough_time() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
}
type Point3d = (f32, f32, f32);

enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

type DifferentialEquation = String;
type EarlyModernistPoem = String;
enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum MyOption<T> {
    None,
    Some(T),
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// An ordered collection of `T`s.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[test]
fn test_binary_tree() {
    use self::BinaryTree::*;
    let _jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
    let _mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));
    let _mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: _jupiter_tree,
        right: _mercury_tree,
    }));
    let _saturn_tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: _mars_tree,
        right: Empty,
    }));
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
        RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}
#[test]
fn test_rough_time_to_english() {
    let future = RoughTime::InTheFuture(TimeUnit::Months, 5);
    assert_eq!(rough_time_to_english(future), "5 months from now");
}

pub fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

#[test]
fn test_describe_point() {
    let desc = describe_point(1, 0);
    assert_eq!(desc, "on the x axis");
}

pub struct Balloon {
    location: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn describe_balloon(b: &Balloon) -> String {
    match b.location {
        Point { x: 0, y: height } => format!("straight up {} meters", height),
        Point { x, y } => format!("at ({}m, {}m)", x, y),
    }
}

#[test]
fn test_describe_balloon() {
    let b = Balloon {
        location: Point { x: 22, y: 33 },
    };
    let desc = describe_balloon(&b);
    let expected = "at (22m, 33m)";
    assert_eq!(desc, expected);
}

#[derive(Debug)]
pub struct Account {
    name: String,
    language: String,
}

pub struct UI {
    name: String,
}

impl UI {
    pub fn greet(&self, name: &String, language: &String) {
        println!("Hello {} in {}", name, language);
    }
    pub fn show_settings(&self, account: &Account) {
        println!("settings are {:?}", account);
    }
}

#[test]
fn test_account() {
    let account = Account {
        name: "Leonty".to_string(),
        language: "Russian".to_string(),
    };
    let ui = UI {
        name: "ui".to_string(),
    };
    match account {
        Account {
            ref name,
            ref language,
            ..
        } => {
            ui.greet(name, language);
            ui.show_settings(&account);
        }
    }
}

pub struct Point3dv2 {
    name: String,
    x: f32,
    y: f32,
    z: f32,
}

pub struct Sphere {
    c: Point3dv2,
    r: f32,
}

impl Sphere {
    pub fn center(&self) -> &Point3dv2 {
        &self.c
    }
}

#[test]
fn test_sphere() {
    let sphere = Sphere {
        c: Point3dv2 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            name: "abc".to_string(),
        },
        r: 5.0,
    };
    match sphere.center() {
        &Point3dv2 { x, y, z, ref name } => {
            println!("{} {} {} {}", x, y, z, name);
        }
    }
}

fn peek() -> Option<&'static char> {
    return Some(&'\n');
}

#[test]
fn test_peek() {
    match peek() {
        Some(&'\n') | Some(&'\r') => {
            println!("newline");
        }
        _ => println!("none"),
    }
}

#[test]
fn test_match_range() {
    let next_char = 'a';
    match next_char {
        '0'...'9' => {
            println!("digit");
        }
        'a'...'z' | 'A'...'Z' => {
            println!("lettter");
        }
        ' ' | '\n' | '\r' => println!("whitespace"),
        _ => println!("something else"),
    }
}

pub fn modulus(n: i32) -> i32 {
    match n {
        n if n > 0 => n,
        n if n < 0 => -n,
        _ => 0,
    }
}

#[test]
fn test_pattern_guards() {
    let a = -5;
    let b = 10;
    let c = 0;
    assert_eq!(modulus(a), 5);
    assert_eq!(modulus(b), 10);
    assert_eq!(modulus(c), 0);
}

#[test]
fn test_at_pattern() {
    let x = 5;
    match x {
        a @ 1..=5 => println!("{} is 1..5", a),
        _ => println!("something else")
    }
}

#[test]
fn test_descructuring() {
    let numbers = [1,2,3,4,5];
    let sum = numbers.iter().fold(0, |a, &num| a + num);
    println!("{}", sum)
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => 
            *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                element: value,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            BinaryTree::NonEmpty(ref mut node) =>
            if value <= node.element {
                node.left.add(value);
            } else {
                node.right.add(value);
            }
        }
    }
}

#[test]
fn test_add() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
}

fn main() {
    println!("Hello, world!");
}

// Chapter complete
