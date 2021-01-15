#[test]
fn test_ops_as_functions() {
    use std::ops::Add;
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}

#[test]
fn test_complex_add1() {
    use num::Complex;
    use std::ops::Add;
    // impl Add for Complex<i32> {
    //     type Output = Complex<i32>;
    //     fn add(self, rhs: Self) -> Self {
    //         Complex {
    //             re: self.re + rhs.re,
    //             im: self.im + rhs.im,
    //         }
    //     }
    // }
}
#[test]
fn test_complex_add2() {
    use num::Complex;
    use std::ops::Add;
    // impl<T> Add for Complex<T> where T: Add<Output=T> {
    //     type Output = Self;
    //     fn add(self, rhs: Self) -> Self {
    //         Complex {
    //             re: self.re + rhs.re,
    //             im: self.im + rhs.im,
    //         }
    //     }
    // }
}

#[test]
fn test_complex_add3() {
    use num::Complex;
    use std::ops::Add;
    // impl<L,R,O> Add<Complex<R>> for Complex<L> where L: Add<R, Output=O> {
    //     type Output = Complex<O>;
    //     fn add(self, rhs: Complex<R>) -> Self::Output {
    //         Complex {
    //             re: self.re + rhs.re,
    //             im: self.im + rhs.im,
    //         }
    //     }
    // }
}

#[test]
fn test_neg_complex() {
    use num::Complex;
    use std::ops::Neg;
    // impl<T, O> Neg for Complex<T> where T: Neg<Output=O> {
    //     type Output = Complex<O>;
    //     fn neg(self) -> Complex<O> {
    //         Complex { -self.re, -self.im }
    //     }
    // }
}

#[test]
fn test_add_assign() {
    use num::Complex;
    use std::ops::AddAssign;

    // impl<T> AddAssign for Complex<T>
    // where T: AddAssign<T> {
    //     fn add_assign(&mut self, rhs: Complex<T>) {
    //         self.re += rhs.re;
    //         self.im += rhs.im;
    //     }
    // }
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

use std::cmp::{Ordering, PartialOrd};

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_partial_ord() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
    // Overlapping intervals aren't ordered with respect to each other.
    let left = Interval {
        lower: 10,
        upper: 30,
    };
    let right = Interval {
        lower: 20,
        upper: 40,
    };
    assert!(!(left < right));
    assert!(!(left >= right));
}

#[test]
fn test_index_hashmap() {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);
    m.insert("万", 1_0000);
    m.insert("億", 1_0000_0000);
    assert_eq!(m["十"], 10);
    assert_eq!(m["千"], 1000);
    use std::ops::Index;
    assert_eq!(*m.index("十"), 10);
    assert_eq!(*m.index("千"), 1000);
}

#[derive(Debug)]
struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    println!("Hello, world!");
}

// Index and IndexMut, page 277
