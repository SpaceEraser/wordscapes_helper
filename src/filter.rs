use serde::{Deserialize, Serialize};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum Filter {
    Permissive,
    Restrictive,
    ExactString(Vec<u8>),
    LengthRestricted(usize),
    Mixed(Vec<u8>),
}

impl Filter {
    pub fn new<S: AsRef<str>>(filter: S) -> Self {
        let filter_lowercase = filter.as_ref().to_lowercase();
        let filter = filter_lowercase.as_bytes();
        let mut processed_filter = Vec::new();
        let mut only_alpha = true;
        let mut only_free = true;

        let mut i = 0;
        while i < filter.len() {
            let mut buf = String::new();
            while i < filter.len() && (filter[i] as char).is_numeric() {
                buf.push(filter[i] as char);
                i += 1;
            }

            if !buf.is_empty() {
                let num: u8 = buf.parse().unwrap();
                processed_filter.extend((0..num).map(|_| b'_'));
                only_alpha = false;

                i -= 1;
            } else if filter[i] == b'-' || filter[i] == b'_' || filter[i] == b'#' {
                processed_filter.push(b'_');
                only_alpha = false;
            } else if (filter[i] as char).is_ascii_alphabetic() {
                processed_filter.push(filter[i]);
                only_free = false;
            }

            i += 1;
        }

        if only_alpha {
            Filter::ExactString(processed_filter)
        } else if only_free {
            Filter::LengthRestricted(processed_filter.len())
        } else {
            Filter::Mixed(processed_filter)
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        match self {
            Filter::Permissive => true,
            Filter::Restrictive => false,
            Filter::ExactString(v) => word.as_bytes() == &v[..],
            Filter::LengthRestricted(len) => word.len() == *len,
            Filter::Mixed(v) => {
                if v.len() != word.len() {
                    return false;
                }

                for (&fc, &wc) in v.iter().zip(word.as_bytes()) {
                    if fc == b'_' {
                        continue;
                    }
                    if !fc.eq_ignore_ascii_case(&wc) {
                        return false;
                    }
                }

                return true;
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Filter::Permissive | Filter::Restrictive => 0,
            Filter::LengthRestricted(len) => *len,
            Filter::ExactString(v) | Filter::Mixed(v) => v.len(),
        }
    }
}
