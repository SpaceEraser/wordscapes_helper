use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct Word(String, usize);

impl Word {
    pub fn from_pair(word: String, freq: usize) -> Self {
        Self(word, freq)
    }

    pub fn from_freqlist_line(line: &str) -> Self {
        let mut words = line.split_ascii_whitespace();
        let (word, freq) = (
            words.next().expect("Failed to get word from line"),
            words.next().expect("Failed to get freq from line"),
        );

        Self(
            word.to_string(),
            freq.parse()
                .unwrap_or_else(|_| panic!("Failed to parse {} as usize", freq)),
        )
    }

    pub fn frequency(&self) -> usize {
        self.1
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Word {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::borrow::Borrow<str> for Word {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}
