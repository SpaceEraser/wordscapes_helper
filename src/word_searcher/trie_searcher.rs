use super::{
    embedded_wordlist_iter, iter_to_wordmap, path_to_iter, str_to_set, AlphaMultiset, BitArray,
    WordSearcher,
};
use crate::Word;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A binary trie based on the bitset representation of a word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrieSearcher {
    trie_root: TrieNode,
}

impl Default for TrieSearcher {
    fn default() -> Self {
        Self::from_embedded_wordlist()
    }
}

impl TrieSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self::from_wordmap(iter_to_wordmap(path_to_iter(path)))
    }

    pub fn from_embedded_wordlist() -> Self {
        Self::from_wordmap(iter_to_wordmap(embedded_wordlist_iter()))
    }

    fn from_wordmap(wordmap: HashMap<AlphaMultiset, Vec<Word>>) -> Self {
        let mut trie_root = TrieNode::default();

        for (set, words) in wordmap {
            trie_root.insert((set, words));
        }

        Self { trie_root }
    }
}

impl WordSearcher for TrieSearcher {
    fn lookup(&self, word: &str) -> Vec<Word> {
        self.trie_root.lookup(str_to_set(word))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct TrieNode {
    words: Vec<Word>,
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    pub fn insert(&mut self, entry: (AlphaMultiset, Vec<Word>)) {
        self.insert_impl(0, (entry.0.into(), entry.1))
    }

    fn insert_impl(&mut self, index: usize, mut entry: (BitArray, Vec<Word>)) {
        if entry.0.is_empty() {
            self.words = entry.1;
            return;
        }

        let bit = entry.0.get(index).unwrap();
        entry.0.set(index, false);

        let child = self.children[bit as usize].get_or_insert_with(Default::default);

        child.insert_impl(index + 1, entry);
    }

    pub fn lookup(&self, set: AlphaMultiset) -> Vec<Word> {
        self.lookup_impl(0, set.into())
    }

    fn lookup_impl(&self, index: usize, mut set: BitArray) -> Vec<Word> {
        // TODO: could we speed this up by looking at the next 2 bits instead of just 1?
        let bit = set.get(index).unwrap();
        set.set(index, false);

        let mut words = self.words.clone();

        let sub_words = if bit {
            self.children
                .iter()
                .filter_map(|c| c.as_ref())
                .flat_map(|c| c.lookup_impl(index + 1, set.clone()))
                .collect()
        } else if let Some(child) = &self.children[0] {
            child.lookup_impl(index + 1, set)
        } else {
            Vec::new()
        };

        words.extend(sub_words);

        return words;
    }
}
