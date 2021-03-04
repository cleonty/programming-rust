#![allow(dead_code)]
#![allow(deprecated)]

extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate unicode_normalization;

fn latin1_to_char(latin1: u8) -> char {
    latin1 as char
}

fn char_to_latin1(c: char) -> Option<u8> {
    if c as u32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}

#[test]
fn test_udon() {
    assert_eq!(
        "うどん: udon".as_bytes(),
        &[
            0xe3, 0x81, 0x86, // う
            0xe3, 0x81, 0xa9, // ど
            0xe3, 0x82, 0x93, // ん
            0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e // : udon
        ]
    );
}

#[test]
fn test_hebrew() {
    assert_eq!("ערב טוב".chars().next(), Some('ע'));
}

#[test]
fn test_digits() {
    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15, 16), Some('f'));
    assert!(char::is_digit('f', 16));
}

#[test]
fn test_case_conversion() {
    let mut upper = 's'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);
    let mut upper = 'ß'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);
    // Unicode says to lowercase Turkish dotted capital 'İ' to 'i'
    // followed by `'\u{307}'`, COMBINING DOT ABOVE, so that a
    // subsequent conversion back to uppercase preserves the dot.
    let ch = 'İ'; // `'\u{130}'`
    let mut lower = ch.to_lowercase();
    assert_eq!(lower.next(), Some('i'));
    assert_eq!(lower.next(), Some('\u{307}'));
    assert_eq!(lower.next(), None);
}

#[test]
fn test_conversion_to_integers() {
    assert_eq!('B' as u32, 66);
    assert_eq!('饂' as u8, 66); // upper bits truncated
    assert_eq!('二' as i8, -116); // same
    assert_eq!(char::from(66), 'B');
    assert_eq!(std::char::from_u32(0x9942), Some('饂'));
    assert_eq!(std::char::from_u32(0xd800), None); // reserved for UTF-16
}

#[test]
fn test_string_slices() {
    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[5..], "eeping");
    assert_eq!(&full[2..4], "ok");
    assert_eq!(full[..].len(), 11);
    assert_eq!(full[5..].contains("boo"), false);
    // index
    let parenthesized = "Rust (饂)";
    assert_eq!(parenthesized[6..].chars().next(), Some('饂'));
}

#[test]
fn test_append_with_write() {
    use std::fmt::Write;
    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap();
    writeln!(letter, "His house is in the village though;").unwrap();
    assert_eq!(
        letter,
        "Whose rutabagas these are I think I know\n\
His house is in the village though;\n"
    );
}

#[test]
fn test_add_and_add_assign() {
    let left = "partners".to_string();
    let mut right = "crime".to_string();
    assert_eq!(left + " in " + &right, "partners in crime");
    right += " doesn't pay";
    assert_eq!(right, "crime doesn't pay");
}

#[test]
fn test_find() {
    let haystack = "One fine day, in the middle of the night";
    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));
}

#[test]
fn test_find_and_replace() {
    let quip = "We also know there are known unknowns";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));
    assert_eq!(
        "The only thing we have to fear is fear itself".replace("fear", "spin"),
        "The only thing we have to spin is spin itself"
    );
    assert_eq!(
        "`Borrow` and `BorrowMut`".replace(|ch: char| !ch.is_alphanumeric(), ""),
        "BorrowandBorrowMut"
    );
}

#[test]
fn test_iterating_over_text() {
    assert_eq!(
        "Лёва".char_indices().collect::<Vec<_>>(),
        vec![
            (0, 'Л'), // has a two-byte UTF-8 encoding
            (2, 'ё'),
            (4, 'в'),
            (6, 'а')
        ]
    );
    assert_eq!(
        "Лёва".bytes().collect::<Vec<_>>(),
        vec![208, 155, 209, 145, 208, 178, 208, 176]
    );
}

#[test]
fn test_split() {
    // The ':' characters are separators here. Note the final "".
    assert_eq!(
        "jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
        vec!["jimb", "1000", "Jim Blandy", ""]
    );
    // The '\n' characters are terminators here.
    assert_eq!(
        "127.0.0.1 localhost\n\
127.0.0.1 www.reddit.com\n"
            .split_terminator('\n')
            .collect::<Vec<_>>(),
        vec!["127.0.0.1 localhost", "127.0.0.1 www.reddit.com"]
    );
    // Note, no final ""!
    let poem = "This is just to say\n\
    I have eaten\n\
    the plums\n\
    again\n";
    assert_eq!(
        poem.split_whitespace().collect::<Vec<_>>(),
        vec!["This", "is", "just", "to", "say", "I", "have", "eaten", "the", "plums", "again"]
    );
}

#[test]
fn test_trimming() {
    assert_eq!("\t*.rs ".trim(), "*.rs");
    assert_eq!("\t*.rs ".trim_start(), "*.rs ");
    assert_eq!("\t*.rs ".trim_end(), "\t*.rs");
    assert_eq!("001990".trim_start_matches('0'), "1990");
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_eq!(usize::from_str("3628800"), Ok(3628800));
    assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(f64::from_str("not a float at all").is_err());
    assert!(bool::from_str("TRUE").is_err());
    use std::net::IpAddr;
    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    assert_eq!(
        address,
        IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50])
    );
}

#[test]
fn test_format() {
    assert_eq!(format!("{}, wow", "doge"), "doge, wow");
    assert_eq!(format!("{}", true), "true");
    assert_eq!(
        format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0) / 2.0),
        "(0.500, 0.866)"
    );
    use std::net::IpAddr;
    use std::str::FromStr;
    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    let formatted_addr: String = format!("{}", address);
    assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");
}

#[test]
fn test_from_utf8() {
    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));
    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    // Since String::from_utf8 failed, it didn't consume the original
    // vector, and the error value hands it back to us unharmed.
    assert_eq!(
        result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]
    );
}

use std::borrow::Cow;
fn get_name() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"))
}

#[test]
fn test_println() {
    println!(
        "{:.3}μs: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object", 140737488346304_usize, 6299664_usize, 64
    );
}

fn format_test_values() {
    let str = "bookends";
    println!("'{}'", str);
    println!("'{:4}'", str);
    println!("'{:12}'", str);
    println!("'{:.4}'", str);
}

fn format_map() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("Portland", (45.5237606, -122.6819273));
    map.insert("Taipei", (25.0375167, 121.5637));
    println!("{:#?}", map);
}

fn print_pointers() {
    use std::rc::Rc;
    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
    println!("text: {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);
}

#[derive(Debug)]
struct Complex {
    r: f64,
    i: f64,
}

use std::fmt;

impl fmt::Display for Complex {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (r, i) = (self.r, self.i);
        if dest.alternate() {
            let abs = f64::sqrt(r * r + i * i);
            let angle = f64::atan2(i, r) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let i_sign = if i < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", r, i_sign, f64::abs(i))
        }
    }
}

use regex::Regex;

#[test]
fn test_regex_basic() {
    // A semver version number, like 0.2.1.
    // May contain a pre-release version suffix, like 0.2.1-alpha.
    // (No build metadata suffix, for brevity.)
    //
    // Note use of r"..." raw string syntax, to avoid backslash blizzard.
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
    // Simple search, with a Boolean result.
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));

    let captures = semver
        .captures(haystack)
        .ok_or("semver regex should have matched")
        .unwrap();
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");
    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");
    let haystack = "In the beginning, there was 1.0.0. \
            For a while, we used 1.0.1-beta, \
            but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver
        .find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);
}

lazy_static! {
    static ref SEMVER: Regex =
        Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("error parsing regex");
}

#[test]
fn test_normalization() {
    assert!("th\u{e9}" != "the\u{301}");
    assert!("th\u{e9}" > "the\u{301}");
    // A Hasher is designed to accumulate the hash of a series of values,
    // so hashing just one is a bit clunky.
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    // These values may change in future Rust releases.
    assert_eq!(hash("th\u{e9}"), 0x53e2d0734eb1dff3);
    assert_eq!(hash("the\u{301}"), 0x90d837f0a0928144);
}
use unicode_normalization::UnicodeNormalization;

#[test]
fn test_nfc_nfd() {
    // No matter what representation the lefthand string uses
    // (you shouldn't be able to tell just by looking),
    // these assertions will hold.
    assert_eq!("Phở".nfd().collect::<String>(), "Pho\u{31b}\u{309}");
    assert_eq!("Phở".nfc().collect::<String>(), "Ph\u{1edf}");
    // The lefthand side here uses the "ffi" ligature character.
    assert_eq!(
        "① Di\u{fb03}culty".nfkc().collect::<String>(),
        "1 Difficulty"
    );
}

fn main() {
    let a = 'А';
    println!("{} to lowercase is {}", a, a.to_lowercase());
    println!("{} as u32 {:#02x}", a, a as u32);
    let name = get_name();
    println!("your name {}", name);
    format_test_values();
    format_map();
    print_pointers();
    /*
    use std::io::BufRead;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some(match_) = SEMVER.find(&line) {
            println!("{}", match_.as_str());
        }
    }
    */
    println!("{:?}", "Лёва".nfd().collect::<String>());
    println!("{:?}", "Лёва".nfc().collect::<String>());
}

// Strings and Text, Normalization on page 427
