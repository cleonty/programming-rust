use std::hash::Hash;
use std::io::Write;
use std::ops::Range;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn test_say_hello() -> std::io::Result<()> {
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;
    let mut bytes = vec![];
    say_hello(&mut bytes)
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_traits_scope() {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello");
}

#[test]
fn test_trait_objects() {
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf;
}

fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

#[test]
fn test_collect() {
    let v1: Vec<i32> = (0..1000).collect();
    let v2 = (0..1000).collect::<Vec<i32>>();
}

use std::fmt::Debug;

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}

fn top_ten2<T>(values: &Vec<T>)
where
    T: Debug + Hash + Eq,
{
}

trait Vegetable {
    fn eat(&self);
}

struct Salad<V: Vegetable> {
    veggies: Vec<V>,
}

struct Salad2 {
    veggies: Vec<Box<dyn Vegetable>>,
}

#[test]
fn test_sink() {
    let mut sink = std::io::sink();
    say_hello(&mut sink);
}

struct Canvas {
    size: (i32, i32),
}

impl Canvas {
    fn write_at(&self, x: i32, y: i32, symbol: char) {}
}

struct Broom {
    name: String,
    height: i32,
    health: i32,
    x: i32,
    y: i32,
}

/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen.
trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);
    /// Return true if clicking at (x, y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

/// A Writer that ignores whatever data you write to it.
pub struct Sink;

use std::io::Result;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        return true;
    }
}

#[test]
fn test_emoji() {
    assert_eq!('x'.is_emoji(), true);
}

use std::io::{self};

struct HtmlDocument;
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, _html: &HtmlDocument) -> io::Result<()> {
        Ok(())
    }
}

trait Clone2 {
    fn clone(&self) -> Self;
}

pub trait MegaSpliceable {
    fn splice(&self, other: &MegaSpliceable) -> Box<MegaSpliceable>;
}

fn splice_megaspliceable(left: &MegaSpliceable, right: &MegaSpliceable) {
    let combo = left.splice(right);
}

struct Direction;

/// Someone in the game world, either the player or some other
/// pixie, gargoyle, squirrel, ogre, etc.
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}

trait StringSet {
    fn new() -> Self
    where
        Self: Sized;
    fn from_slice(string: &[&str]) -> Self
    where
        Self: Sized;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

struct SortedStringSet;

impl StringSet for SortedStringSet {
    fn new() -> Self {
        let s = SortedStringSet {};
        return s;
    }
    fn from_slice(_string: &[&str]) -> Self {
        let s = SortedStringSet {};
        return s;
    }
    fn contains(&self, _string: &str) -> bool {
        return true;
    }

    fn add(&mut self, _string: &str) {}
}

fn unknown_words<S: StringSet>(document: &Vec<String>, worldlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !worldlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

#[test]
fn test_to_string() {
    "hello".to_string();
    str::to_string("hello");
    ToString::to_string("hello");
    <str as ToString>::to_string("hello");
}

use std::iter::Iterator;
fn collect_into_vector<T: Iterator>(iter: T) -> Vec<T::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    return results;
}

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump2<I>(iter: I)
where
    I: Iterator<Item = String>,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

trait Mulx<RHS = Self> {
    type Output;
    fn mul(self, rhs: RHS) -> Self::Output;
}

impl Mulx for i32 {
    type Output = i32;
    fn mul(self, rhs: i32) -> i32 {
        return self * rhs;
    }
}

#[test]
fn test_random() {
    use rand::random;
    let _x = random::<f64>();
}

fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

use std::ops::{Add, Mul};

fn dot2<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot2() {
    assert_eq!(dot2(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot2(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

use num::Num;

fn dot3<N>(v1: &[N], v2: &[N]) -> N
where
    N: Num + Copy,
{
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn test_dot3() {
    assert_eq!(dot3(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot3(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

fn main() {
    println!("Hello, world!");
}

// done