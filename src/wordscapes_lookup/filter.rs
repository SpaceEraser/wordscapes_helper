use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default, Serialize, Deserialize)]
pub struct Filter(String);

impl Filter {
    pub fn new<S: AsRef<str>>(filter: S) -> Self {
        let filter = filter.as_ref().as_bytes();
        let mut processed_filter = String::new();

        let mut i = 0;
        while i < filter.len() {
            let mut buf = String::new();
            while i < filter.len() && (filter[i] as char).is_numeric() {
                buf.push(filter[i] as char);
                i += 1;
            }

            if !buf.is_empty() {
                let num: u8 = buf.parse().unwrap();
                processed_filter.extend((0..num).map(|_| '_'));

                i -= 1;
            } else if filter[i] == b'-' || filter[i] == b'_' {
                processed_filter.push('_');
            } else if (filter[i] as char).is_ascii_alphabetic() {
                processed_filter.push((filter[i] as char).to_ascii_lowercase());
            }
            
            i += 1;
        }

        Self(processed_filter)
    }

    pub fn matches(&self, word: &str) -> bool {
        if self.len() != word.len() { return false; }

        for (fc, wc) in self.0.chars().zip(word.chars()) {
            if fc == '_' { continue }
            if !fc.eq_ignore_ascii_case(&wc) { return false; }
        }
        
        return true;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
