use super::{
    embedded_wordlist_iter, iter_to_wordmap, path_to_iter, str_to_set, AlphaMultiset, WordSearcher,
};
use crate::Word;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpSearcher {
    wordmap: HashMap<AlphaMultiset, Vec<Word>>,
}

impl Default for ExpSearcher {
    fn default() -> Self {
        Self::from_embedded_wordlist()
    }
}

impl ExpSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self {
            wordmap: iter_to_wordmap(path_to_iter(path)),
        }
    }
    pub fn from_embedded_wordlist() -> Self {
        Self {
            wordmap: iter_to_wordmap(embedded_wordlist_iter()),
        }
    }
}

impl WordSearcher for ExpSearcher {
    /// Enumerate all unique subsets of the word multiset and do a table lookup on each of them
    fn lookup(&self, word: &str) -> Vec<Word> {
        let letter_set = str_to_set(word);
        let mut words = Vec::new();

        for subset in enum_subsets(letter_set) {
            if let Some(matches) = self.wordmap.get(&subset) {
                words.extend_from_slice(matches);
            }
        }

        return words;
    }
}

fn enum_subsets(set: AlphaMultiset) -> Vec<AlphaMultiset> {
    enum_subsets_impl(&mut set.char_counts(), 0)
}

fn enum_subsets_impl(counts: &mut [u8; 26], mut index: usize) -> Vec<AlphaMultiset> {
    while index < counts.len() && counts[index] == 0 {
        index += 1;
    }
    if index >= counts.len() {
        return vec![AlphaMultiset::from(&*counts)];
    }

    let mut subsets = Vec::new();
    let ocount = counts[index];

    for c in 0..=ocount {
        counts[index] = c;
        subsets.extend(enum_subsets_impl(counts, index + 1));
    }

    return subsets;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_enum() {
        let mut subs = enum_subsets(AlphaMultiset::from("abc"))
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        subs.sort();

        assert_eq!(subs, vec!["", "a", "ab", "abc", "ac", "b", "bc", "c"]);
    }

    #[test]
    fn test_simple_enum2() {
        let mut subs = enum_subsets(AlphaMultiset::from("aaac"))
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        subs.sort();

        assert_eq!(subs, vec!["", "a", "aa", "aaa", "aaac", "aac", "ac", "c"]);
    }
}
