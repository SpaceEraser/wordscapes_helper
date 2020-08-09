use super::{str_to_set, wordlist_to_wordmap, AlphaMultiset, WordscapesHelper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleSearcher {
    wordmap: HashMap<AlphaMultiset, Vec<String>>,
}

impl SimpleSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self {
            wordmap: wordlist_to_wordmap(path),
        }
    }
}

impl WordscapesHelper for SimpleSearcher {
    /// Do a linear lookup over all dictionary words for a match
    fn lookup(&self, word: &str) -> Vec<String> {
        let letter_set = str_to_set(word);
        let mut words = Vec::new();

        for (set, strs) in &self.wordmap {
            if letter_set.has_subset(&set) {
                words.extend_from_slice(&*strs);
            }
        }

        return words;
    }
}
