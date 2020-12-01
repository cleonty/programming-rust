#[test]
fn test_match() {
    let code = 10;
    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code),
    }
}

#[test]
fn test_for_in() {
    let mut last = 0;
    for i in 0..20 {
        last = i;
    }
    assert_eq!(last, 19);
}

#[test]
fn test_for_ref() {
    let mut strings = vec!["one".to_string(), "two".to_string()];
    for rs in &mut strings {
        rs.insert_str(0, "[");
        rs.push_str("]");
    }
    for rs in &strings {
        println!("string {:?} is at address {:p}.", *rs, rs);
    }
}

#[test]
fn test_closures() {
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    println!("{}", is_even(2));
}

fn main() {
    println!("Hello, world!");
}
