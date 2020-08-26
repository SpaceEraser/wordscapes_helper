use crate::{Filter, Word};
use std::collections::HashMap;

mod alpha_multiset;

mod automaton_searcher;
mod dag_searcher;
mod exp_searcher;
mod simple_searcher;
mod trie_searcher;

pub use alpha_multiset::*;

pub use automaton_searcher::*;
pub use dag_searcher::*;
pub use exp_searcher::*;
pub use simple_searcher::*;
pub use trie_searcher::*;

static EMBEDDED_WORDLIST: &[u8] = include_bytes!("freq_200k.txt");

pub trait WordSearcher {
    fn lookup(&self, word: &str) -> Vec<Word>;

    fn lookup_filter(&self, word: &str, filter: &str) -> Vec<Word> {
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

fn embedded_wordlist_iter() -> impl Iterator<Item = String> {
    use std::io::BufRead;

    std::io::BufReader::new(EMBEDDED_WORDLIST)
        .lines()
        .map(Result::unwrap)
}

fn path_to_iter<P: AsRef<std::path::Path>>(path: P) -> impl Iterator<Item = String> {
    use std::io::BufRead;

    let path = path.as_ref();
    let f =
        std::fs::File::open(path).unwrap_or_else(|_| panic!("Unable to find '{}'", path.display()));

    std::io::BufReader::new(f).lines().map(Result::unwrap)
}

fn iter_to_wordmap<I>(words: I) -> HashMap<AlphaMultiset, Vec<Word>>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    // remove non-letter characters and filter words < 3 characters long
    let wordlist = words
        .filter(|l| !l.as_ref().is_empty())
        .map(|l| Word::from_freqlist_line(l.as_ref()))
        .filter_map(|w| {
            let s: String = w
                .chars()
                .filter(char::is_ascii_alphabetic)
                .map(|c| c.to_ascii_lowercase())
                .collect();

            if s.len() >= 3 {
                Some(Word::from_pair(s, w.frequency()))
            } else {
                None
            }
        });

    let mut wordmap = HashMap::new();

    // construct a map from `AlphaMultiset` to strings which created such sets
    // aka group anagrams and key them by some normal representation
    for w in wordlist {
        let w_norm = str_to_set(w.as_ref());
        wordmap.entry(w_norm).or_insert_with(Vec::new).push(w);
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
