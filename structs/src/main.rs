/// A rectangle of eight-bit grayscale pixels.
#[derive(Debug)]
pub struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

pub fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

#[test]
fn test_construct_grayscale_map() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    println!("image is {:?}", image);
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    // Initialize `broom1` mostly from `b`, changing only `height`. Since
    // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    // Initialize `broom2` mostly from `broom1`. Since `String` is not
    // `Copy`, we must clone `name` explicitly
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

#[test]
fn test_chop() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
}

struct Bounds(usize, usize);
pub struct UBounds(pub usize, pub usize);

#[test]
fn test_bounds() {
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}

struct Onesuch;
#[test]
fn test_one_such() {
    Onesuch;
}

/// A last-in, first-out queue of characters.
pub struct Queue<T> {
    older: Vec<T>,   // older elements, eldest last.
    younger: Vec<T>, // younger elements, youngest last.
}

impl<T> Queue<T> {
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }
    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements in younger over to older, and put them in
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

#[test]
fn test_queue() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
}

#[test]
fn test_queue_split() {
    let mut q = Queue::new();
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}

#[test]
fn test_queue_explict() {
    let mut q = Queue::<&str>::new();
    let mut r = Queue::new();
    q.push("CAD"); // apparently a Queue<&'static str>
    r.push(0.74); // apparently a Queue<f64>
    q.push("BTC"); // Bitcoins per USD, 2017-5
    r.push(2737.7); // Rust fails to detect irrational exuberance
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

fn find_extrema2(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct FileDesc {
    desc: String,
}

struct Camera {}

use std::cell::Cell;
use std::cell::RefCell;
use std::fs::File;

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    log_devices: [FileDesc; 8],
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

use std::rc::Rc;

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    eyes: [Camera; 32],
}

impl SpiderRobot {
    /// Increase the error count by 1.
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }
    /// True if any hardware errors have been reported.
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
    
    /// Write a line to the log file.
    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}

#[test]
fn test_ref_cell() {
    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }
    {
        let mut w = ref_cell.borrow_mut(); // panic: already borrowed
        w.push_str(" world");
    }
}

fn main() {
    println!("Hello, world!");
}
