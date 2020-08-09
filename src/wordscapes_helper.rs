use std::collections::HashMap;

mod alpha_multiset;
mod filter;

mod dag_searcher;
mod exp_searcher;
mod simple_searcher;
mod trie_searcher;

pub use alpha_multiset::*;
pub use filter::*;

pub use dag_searcher::*;
pub use exp_searcher::*;
pub use simple_searcher::*;
pub use trie_searcher::*;

pub trait WordscapesHelper {
    fn lookup(&self, word: &str) -> Vec<String>;

    fn lookup_filter(&self, word: &str, filter: &str) -> Vec<String> {
        let lookup = self.lookup(word);

        if filter.is_empty() {
            lookup
        } else {
            let filter = Filter::new(filter);

            lookup
                .into_iter()
                .filter(|word| filter.matches(word))
                .collect()
        }
    }
}

fn wordlist_to_wordmap<P: AsRef<std::path::Path>>(path: P) -> HashMap<AlphaMultiset, Vec<String>> {
    let path = path.as_ref();
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

    let mut wordmap = HashMap::new();

    // construct a map from `AlphaMultiset` to strings which created such sets
    // aka group anagrams and key them by some normal representation
    for w in wordlist {
        let w = w.as_ref();
        let w_norm = str_to_set(w);
        wordmap
            .entry(w_norm)
            .or_insert_with(Vec::new)
            .push(w.to_string());
    }

    return wordmap;
}

fn str_to_set(word: &str) -> AlphaMultiset {
    std::iter::FromIterator::from_iter(
        word.chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase()),
    )
}
