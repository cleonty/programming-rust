use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

#[allow(dead_code)]
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

#[allow(dead_code)]
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[test]
fn test_show() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    sort_works(&mut table);
    show(&table);
    assert_eq!(table["Gesualdo"][1], "many madrigals");
}

#[test]
fn test_ref_as_val() {
    let x = 10;
    let r = &x;
    assert!(*r == 10);
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    assert!(y == 64);

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    };
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!(anime_ref.bechdel_pass, true);
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

#[test]
fn test_assign_ref() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;

    if b {
        r = &y;
    }
    assert!(*r == 10 || *r == 20);
}

#[test]
fn test_ref_to_ref() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.x, 1000);
    assert_eq!(rrr.y, 729);
}

#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

#[test]
fn test_borrowing_refs() {
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729)
}

#[test]
fn test_borrowing_local_variable() {
    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1);
        }
        // assert_eq!(*r, 1);
    }
}

#[allow(dead_code)]
static mut STASH: &i32 = &128;

#[allow(dead_code)]
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

#[test]
fn test_static() {
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}

#[allow(dead_code)]
fn g<'a>(p: &'a i32) {
    println!("{}", p);
}

#[test]
fn test_g() {
    let x = 10;
    g(&x);
}

// v should have at least one element
#[allow(dead_code)]
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

#[test]
fn fn_smallest() {
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
}

#[test]
fn test_refs_in_structs() {
    #[derive(Debug)]
    struct S<'a> {
        r: &'a i32,
    }
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
}

#[test]
fn test_non_distinct_lifetime_parameters() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
            println!("r {}", r);
            println!("y {}", s.y);
        }
    }
}

#[derive(Debug)]
struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
#[allow(dead_code)]
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

#[test]
fn test_sharing_versus_mutation() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
        
    }
    let _aside = v;
}

#[allow(dead_code)]
fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

#[test]
fn test_extend() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    
    extend(&mut wave, &wave);
}

fn main() {
    println!("Hello, world!");
}
