use std::collections::HashMap;

use wordscapes_helper::*;

fn main() {
    let letters = if let Some(letters) = std::env::args().nth(1) {
        letters
    } else {
        println!(
            "Usage: {} <characters to look up>",
            std::env::args().nth(0).unwrap()
        );
        std::process::exit(1)
    };
    let letter_set = AlphaMultiset::from(&*letters);

    let start = std::time::Instant::now();
    let word_map = construct_wordmap("wordlist.txt");
    let elapsed = start.elapsed();

    println!("Wordmap construction took {:?}", elapsed);
    let mut words = Vec::new();

    let start = std::time::Instant::now();
    for (set, strs) in word_map {
        if letter_set.has_subset(&set) {
            words.extend_from_slice(&*strs);
        }
    }
    let elapsed = start.elapsed();

    println!("Words that can be made from '{}':", letters);
    for word in &words {
        println!("\t{}", word);
    }
    println!("Found {} items", words.len());
    println!("Lookup took {:?}", elapsed);
}

fn construct_wordmap(path: &str) -> HashMap<AlphaMultiset, Vec<String>> {
    let path = std::path::Path::new(path);
    let words = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to find '{}'", path.display()));

    // remove non-letter characters and filter words < 3 characters long
    let wordlist = words.trim().lines().filter_map(|s| {
        let s: String = s
            .chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if s.len() >= 3 {
            Some(s)
        } else {
            None
        }
    });

    let mut word_map = HashMap::new();

    for w in wordlist {
        let w = w.as_ref();
        let w_norm = str_to_set(w);
        word_map
            .entry(w_norm)
            .or_insert_with(Vec::new)
            .push(w.to_string());
    }

    return word_map;
}

fn str_to_set(word: &str) -> AlphaMultiset {
    std::iter::FromIterator::from_iter(
        word.chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase()),
    )
}
