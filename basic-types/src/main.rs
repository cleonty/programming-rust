#[test]
fn test_overflow_1() {
    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1);
    assert_eq!(x, std::i32::MIN);
}

#[test]
fn test_overflow_2() {
    let mut val: i32 = std::i32::MAX;
    while val > 0 {
        val = val.wrapping_add(1);
    }
    assert_eq!(val, std::i32::MIN);
}

#[test]
fn test_numbers() {
    let x = 123_456_789_u32;
    assert_eq!(x, 123456789);
}

#[test]
fn test_floats() {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((1f32 / 3f32) * 3f32, 1.);
}

#[test]
fn test_char() {
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10).unwrap(), 8);
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}
#[test]
fn test_tuple1() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_array() {
    let x = [1, 2, 3, 4, 5];
    println!("{:?}", x);
    let v = [true; 1000];
    assert_eq!(v.len(), 1000);
    assert_eq!(v[55], true);

    let mut chaos = [3, 4, 1, 5, 7, 6, 8, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_vector() {
    let mut v = vec![2, 3, 4, 5];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 120);
    v.push(6);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 120 * 6);
}

#[test]
fn test_slice() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v[1..];
    let sa: &[f64] = &a[2..];
    assert_eq!(sv[0], 0.707);
    assert_eq!(sa[0], -1.00);
}

#[test]
fn test_str_len() {
    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);
}

#[test]
fn test_str_uppercase() {
    assert_ne!(("Лёва").to_ascii_uppercase(), "ЛЁВА"); // only ascii supported
}

#[test]
fn test_format() {
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());
}

#[test]
fn test_join_strings() {
   let bits = vec!["veni", "vidi", "vici"];
   assert_eq!(bits.concat(), "venividivici");
   assert_eq!(bits.join(", "), "veni, vidi, vici"); 
}

fn main() {
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
