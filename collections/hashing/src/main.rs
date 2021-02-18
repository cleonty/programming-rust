extern crate fnv;

use std::collections::HashSet;
use core::hash::Hash;
use fnv::{FnvHashMap, FnvHashSet};


#[derive(Clone, PartialEq, Eq, Hash)]
enum MuseumNumber {
    Item,
}

struct Culture {}
struct RoughTime {}
struct Artifact {
    id: MuseumNumber,
    name: String,
    cultures: Vec<Culture>,
    date: RoughTime,
}

impl PartialEq for Artifact {
    fn eq(&self, other: &Artifact) -> bool {
        self.id == other.id
    }
}

impl Eq for Artifact {}

impl Hash for Artifact {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(hasher);
    }
}

fn main() {
    let mut collection = HashSet::<Artifact>::new();
    let mut fnv_hash_map = FnvHashMap::<String, String>::default();
    fnv_hash_map.insert("a".to_string(), "b".to_string());
}

// Collections, Hashing on page 387
