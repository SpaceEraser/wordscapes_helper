use super::{str_to_set, wordlist_to_wordmap, AlphaMultiset, BitArray, WordscapesHelper};
use serde::{Deserialize, Serialize};

/// A binary trie based on the bitset representation of a word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrieSearcher {
    trie_root: TrieNode,
}

impl TrieSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        let wordmap = wordlist_to_wordmap(path);

        let mut trie_root = TrieNode::default();

        for (set, words) in wordmap {
            trie_root.insert((set, words));
        }

        Self { trie_root }
    }
}

impl WordscapesHelper for TrieSearcher {
    fn lookup(&self, word: &str) -> Vec<String> {
        self.trie_root.lookup(str_to_set(word))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct TrieNode {
    words: Vec<String>,
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    pub fn insert(&mut self, entry: (AlphaMultiset, Vec<String>)) {
        self.insert_impl(0, (entry.0.into(), entry.1))
    }

    fn insert_impl(&mut self, index: usize, mut entry: (BitArray, Vec<String>)) {
        if entry.0.is_empty() {
            self.words = entry.1;
            return;
        }

        let bit = entry.0.get(index).unwrap();
        entry.0.set(index, false);

        let child = self.children[bit as usize].get_or_insert_with(Default::default);

        child.insert_impl(index + 1, entry);
    }

    pub fn lookup(&self, set: AlphaMultiset) -> Vec<String> {
        self.lookup_impl(0, set.into())
    }

    fn lookup_impl(&self, index: usize, mut set: BitArray) -> Vec<String> {
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
