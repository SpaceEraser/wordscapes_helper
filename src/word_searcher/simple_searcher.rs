use super::{
    embedded_wordlist_iter, iter_to_wordmap, path_to_iter, str_to_set, AlphaMultiset, WordSearcher,
};
use crate::Word;
use serde::{Deserialize, Serialize};
use fnv::FnvHashMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleSearcher {
    length_inds: FnvHashMap<usize, usize>,
    sorted_ind_keys: Vec<usize>,
    words: Vec<(AlphaMultiset, Vec<Word>)>,
}

impl Default for SimpleSearcher {
    fn default() -> Self {
        Self::from_embedded_wordlist()
    }
}

impl SimpleSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        Self::from_wordmap(iter_to_wordmap(path_to_iter(path)))
    }

    pub fn from_embedded_wordlist() -> Self {
        Self::from_wordmap(iter_to_wordmap(embedded_wordlist_iter()))
    }

    fn from_wordmap(map: HashMap<AlphaMultiset, Vec<Word>>) -> Self {
        let mut words: Vec<_> = map.into_iter().collect();
        words.sort_unstable_by_key(|(s, _)| -(s.len() as isize));
        
        let min_length = words.last().unwrap().0.len();
        let max_length = words.first().unwrap().0.len();
        let mut cur_length = min_length;
        let mut length_inds = FnvHashMap::default();

        for (i, (set, _)) in words.iter().enumerate().rev() {
            let l = set.len();

            if l > cur_length {
                length_inds.insert(cur_length, i + 1);
                cur_length = l;
            }
        }
        length_inds.insert(max_length, 0);

        let mut sorted_ind_keys: Vec<_> = length_inds.keys().cloned().collect();
        sorted_ind_keys.sort_unstable();

        Self {
            length_inds,
            words,
            sorted_ind_keys,
        }
    }

    fn find_closest_index_key(&self, n: usize) -> usize {
        match self.sorted_ind_keys.binary_search(&n) {
            Ok(_) => n,
            Err(closest_ind) => {
                self.sorted_ind_keys[closest_ind.clamp(0, self.sorted_ind_keys.len() - 1)]
            }
        }
    }
}

impl WordSearcher for SimpleSearcher {
    /// Do a linear lookup over dictionary words with length <= the given word
    fn lookup(&self, word: &str) -> Vec<Word> {
        let index_len = self.find_closest_index_key(word.len());
        let start_ind = *self.length_inds.get(&index_len).unwrap();
        let letter_set = str_to_set(word);
        let mut words = Vec::new();

        for (set, strs) in &self.words[start_ind..] {
            if letter_set.has_subset(&set) {
                words.extend_from_slice(&*strs);
            }
        }

        return words;
    }
}
