use super::{
    embedded_wordlist_iter, iter_to_wordmap, path_to_iter, str_to_set, AlphaMultiset, WordSearcher,
};
use crate::Word;
use regex::bytes::{RegexSet, RegexSetBuilder};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AutomatonSearcher {
    rset: RegexSet,
    words: Vec<Vec<Word>>,
}

impl Default for AutomatonSearcher {
    fn default() -> Self {
        Self::from_embedded_wordlist()
    }
}

impl AutomatonSearcher {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        let (rset, words) = build_regex_set(iter_to_wordmap(path_to_iter(path)));
        Self { rset, words }
    }
    pub fn from_embedded_wordlist() -> Self {
        let (rset, words) = build_regex_set(iter_to_wordmap(embedded_wordlist_iter()));
        Self { rset, words }
    }
}

impl WordSearcher for AutomatonSearcher {
    fn lookup(&self, word: &str) -> Vec<Word> {
        let norm_word = str_to_set(word).to_string();

        self.rset
            .matches(norm_word.as_bytes())
            .into_iter()
            .flat_map(|i| self.words[i].clone())
            .collect()
    }
}

fn build_regex_set(wordmap: HashMap<AlphaMultiset, Vec<Word>>) -> (RegexSet, Vec<Vec<Word>>) {
    let (sets, words): (Vec<_>, Vec<_>) = wordmap.into_iter().unzip();

    let rset = RegexSetBuilder::new(sets.into_iter().map(|s| s.to_string()).map(|mut s| {
        for i in (0..=s.len()).rev() {
            s.insert_str(i, r"[a-z]*");
        }
        return s;
    }))
    .size_limit(2 << 28)
    .unicode(false)
    .build()
    .expect("failed to build RegexSet");

    return (rset, words);
}
