use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use normword::*;

mod normword;

static EMBEDDED_WORDLIST_BYTES: &'static [u8] = include_bytes!("dag.bin");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordscapesLookup {
    dag: DiGraph<(AlphaMultiset, Vec<String>), ()>,
}

impl Default for WordscapesLookup {
    fn default() -> Self {
        Self::from_embedded_wordlist()
    }
}

impl WordscapesLookup {
    pub fn from_wordlist<P: AsRef<std::path::Path>>(path: P) -> Self {
        let path = path.as_ref();
        let words = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Unable to find '{}'", path.display()));

        // remove non-letter characters and filter words < 3 characters long
        Self::from_words(words.trim().lines().filter_map(|s| {
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
        }))
    }

    pub fn from_words<'a, I>(wordlist: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut word_map = HashMap::new();

        for w in wordlist {
            let w = w.as_ref();
            let w_norm = str_to_set(w);
            word_map
                .entry(w_norm)
                .or_insert_with(Vec::new)
                .push(w.to_string());
        }

        Self {
            dag: build_dag(word_map),
        }
    }

    pub fn from_embedded_wordlist() -> Self {
        bincode::deserialize(EMBEDDED_WORDLIST_BYTES).unwrap()
    }

    pub fn lookup(&self, word: &str) -> Vec<String> {
        let norm = str_to_set(word);
        // println!("looking up word {} => {}", word, norm);

        let full_word_node = NodeIndex::from(0);

        let mut visited = fixedbitset::FixedBitSet::with_capacity(self.dag.node_count());
        let mut bfs_queue = std::collections::VecDeque::new();
        let mut found_words = Vec::new();

        bfs_queue.push_back(full_word_node);

        while let Some(nx) = bfs_queue.pop_front() {
            if visited.contains(nx.index()) {
                continue;
            }
            visited.put(nx.index());

            if norm.has_subset(&self.dag[nx].0) {
                found_words.extend_from_slice(&*self.dag[nx].1);
                bfs_queue.extend(self.dag.neighbors(nx));
            } else if self.dag[nx].0.count_common(&norm) >= 3 {
                bfs_queue.extend(self.dag.neighbors(nx));
            }
        }

        found_words.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));

        return found_words;
    }
}

fn str_to_set(word: &str) -> AlphaMultiset {
    std::iter::FromIterator::from_iter(
        word.chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase()),
    )
}

// Build a lookup DAG with special root node at index 0 for easy lookups
fn build_dag(mut words: HashMap<AlphaMultiset, Vec<String>>) -> DiGraph<(AlphaMultiset, Vec<String>), ()> {
    let mut dag = DiGraph::<(AlphaMultiset, Vec<String>), ()>::default();
    let full_word_node = dag.add_node((AlphaMultiset::new_full(), Vec::new()));

    let mut i = 0;
    while !words.is_empty() {
        if i > 0 && i % 10_000 == 0 {
            println!("Inserted {} words", i);
        }
        i += 1;

        // find word which is not contained in any other word
        let best = {
            let mut word_iter = words.keys();
            let mut best = word_iter.next().unwrap();
            while let Some(cur) = word_iter.next() {
                if cur.has_subset(best) {
                    best = cur;
                }
            }
            best.clone()
        };

        // finds shortest words that still contain "best" as an anagram subsequence
        let mut visited = fixedbitset::FixedBitSet::with_capacity(dag.node_count());
        let mut bfs_queue = std::collections::VecDeque::new();
        let mut containing_nodes = Vec::new();
        bfs_queue.push_back(full_word_node);

        while let Some(nx) = bfs_queue.pop_front() {
            if visited.contains(nx.index()) {
                continue;
            }
            visited.put(nx.index());

            if !dag.neighbors(nx).any(|ni| dag[ni].0.has_subset(&best)) {
                containing_nodes.push(nx);
            } else {
                bfs_queue.extend(dag.neighbors(nx).filter(|&ni| dag[ni].0.has_subset(&best)));
            }
        }

        // add "best" to DAG
        let new_nx = dag.add_node(words.remove_entry(&best).unwrap());

        // connect shortest words found above to "best" node
        for parent in containing_nodes {
            dag.add_edge(parent, new_nx, ());
        }
    }

    return dag;
}
