use std::collections::HashMap;
use std::collections::HashSet;

fn read_words(file: &str) -> Vec<String> {
    vec!["abc", "abcd", "abcde"]
        .into_iter()
        .map(|x| format!("{}-{}", file, x))
        .collect()
}

fn main() {
    let files: Vec<String> = vec!["a", "b", "c"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let mut word_occurrence: HashMap<String, HashSet<String>> = HashMap::new();
    for file in files {
        for word in read_words(&file) {
            let set = word_occurrence.entry(word).or_insert_with(HashSet::new);
            set.insert(file.clone());
        }
    }
    println!("{:?}", word_occurrence);
}

// Collections, HashMap<K, V> and BTreeMap<K, V> on page 378
