fn main() {
    print_padovan();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here

#[test]
fn test_box() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}

fn f(_v: Vec<i32>) {}

fn g(_v: Vec<i32>) {}

#[test]
fn test_move_if() {
    let x = vec![10, 20, 30];
    const C: i32 = 5;
    if C > 4 {
        f(x); // ... ok to move from x here
    } else {
        g(x); // ... and ok to also move from x here
    }
    // h(x); // bad: x is uninitialized here if either path uses it
}

#[test]
fn test_move_vector() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(v, vec!["101", "104", "substitute"]);
    let new_first = v.remove(0);
    assert_eq!(new_first, "101");
    assert_eq!(v, vec!["104", "substitute"]);
}

#[test]
fn test_vec_2() {
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

#[test]
fn test_move_value_out_of_owner() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    println!("{:?}", composers);
    //let first_name = composers[0].name;
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    let first_name_2 = composers[0].name.take();
    assert_eq!(first_name_2, None);
}

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}
fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

#[test]
fn test_non_copy_types() {
    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

use std::rc::Rc;

#[test]
fn test_rc() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
