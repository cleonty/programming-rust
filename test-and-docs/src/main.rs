#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[should_panic(expected = "divide by zero")]
#[allow(unconditional_panic)]
fn test_divide_by_zero() {
    let a = 1;
    let b = 0;
    let c = a / b;
    println!("{}", c);
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }
    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
fn main() {
    println!("Hello, world!");
}
