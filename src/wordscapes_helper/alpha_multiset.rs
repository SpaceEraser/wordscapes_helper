use serde::{Deserialize, Serialize};

const NUM_BLOCKS: usize = 4;
const BLOCK_SIZE: usize = 64;
const A: usize = 'a' as usize;
const Z: usize = 'z' as usize;
const MAX_CHAR_REP: usize = NUM_BLOCKS * BLOCK_SIZE / 26;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Serialize, Deserialize)]
struct BitArray([u64; NUM_BLOCKS]);

impl BitArray {
    /// Create BitArray with all ones
    pub fn new_ones() -> Self {
        Self([u64::MAX, u64::MAX, u64::MAX, u64::MAX])
    }

    /// Get specific bit
    pub fn get(&self, i: usize) -> Option<bool> {
        if i >= NUM_BLOCKS * BLOCK_SIZE {
            None
        } else {
            let b = i / BLOCK_SIZE;
            let n = i % BLOCK_SIZE;
            Some((self.0[b] >> n) & 0b1 == 1)
        }
    }

    /// Get specific bit, with no safety checks
    pub unsafe fn get_unchecked(&self, i: usize) -> bool {
        let b = i / BLOCK_SIZE;
        let n = i % BLOCK_SIZE;
        (self.0.get_unchecked(b) >> n) & 0b1 == 1
    }

    /// Set specific bit
    pub fn set(&mut self, i: usize, val: bool) {
        if i >= NUM_BLOCKS * BLOCK_SIZE {
            return;
        }

        let b = i / BLOCK_SIZE;
        let n = i % BLOCK_SIZE;

        self.0[b] = (self.0[b] & !(1 << n)) | ((val as u64) << n);
    }

    /// Check if `self` has `other` as a subset (not strict)
    pub fn has_subset(&self, other: &Self) -> bool {
        let bs = |a, b| a | b == a;

        bs(self.0[0], other.0[0])
            && bs(self.0[1], other.0[1])
            && bs(self.0[2], other.0[2])
            && bs(self.0[3], other.0[3])
    }

    /// Check if `self` is disjoint from `other`
    pub fn is_disjoint(&self, other: &Self) -> bool {
        let bs = |a, b| a & b == 0;

        bs(self.0[0], other.0[0])
            && bs(self.0[1], other.0[1])
            && bs(self.0[2], other.0[2])
            && bs(self.0[3], other.0[3])
    }

    /// Return how many elements `self` has in common with `other`
    pub fn count_common(&self, other: &Self) -> u32 {
        let bs = |a: u64, b: u64| (a & b).count_ones();

        bs(self.0[0], other.0[0])
            + bs(self.0[1], other.0[1])
            + bs(self.0[2], other.0[2])
            + bs(self.0[3], other.0[3])
    }
}

impl std::fmt::Binary for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..NUM_BLOCKS * BLOCK_SIZE {
            unsafe {
                write!(
                    f,
                    "{}{}",
                    if i > 0 && i % 8 == 0 { " " } else { "" },
                    if self.get_unchecked(i) { 1 } else { 0 }
                )?
            }
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Serialize, Deserialize)]
pub struct AlphaMultiset(BitArray);

impl AlphaMultiset {
    /// Return a set which will have any other set as a subset
    pub fn new_universal() -> Self {
        Self(BitArray::new_ones())
    }

    /// Return a set which will be a subset of any other set
    pub fn new_empty() -> Self {
        Self::default()
    }

    /// Check if `self` has `other` as a subset (not strict)
    pub fn has_subset(&self, other: &Self) -> bool {
        self.0.has_subset(&other.0)
    }

    /// Check if `self` is disjoint from `other`
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    /// Return how many elements `self` has in common with `other`
    pub fn count_common(&self, other: &Self) -> u32 {
        self.0.count_common(&other.0)
    }
}

impl std::convert::From<&[u8; 26]> for AlphaMultiset {
    fn from(alpha_vec: &[u8; 26]) -> Self {
        let mut this = Self::default();

        for (i, c) in alpha_vec.iter().enumerate() {
            if *c > MAX_CHAR_REP as _ {
                panic!(
                    "Can only handle {} repetitions of chars, but '{}' occurs {} times",
                    MAX_CHAR_REP,
                    ((i as u8) + ('a' as u8)) as char,
                    c
                );
            }
            for v in 0..*c {
                this.0.set(i * MAX_CHAR_REP + (v as usize), true);
            }
        }

        this
    }
}

impl<'a> std::convert::From<&'a str> for AlphaMultiset {
    fn from(word: &'a str) -> Self {
        std::iter::FromIterator::from_iter(word.chars())
    }
}

impl std::iter::FromIterator<char> for AlphaMultiset {
    fn from_iter<I: IntoIterator<Item = char>>(words: I) -> Self {
        let mut long_vec = <[u8; 26]>::default();

        for c in words {
            let cu = c as usize;
            if cu < A || cu > Z {
                panic!(
                    "Can only handle lowercase ASCII alpha chars, but got '{}'",
                    c
                );
            }

            long_vec[cu - A] += 1;
        }

        From::from(&long_vec)
    }
}

impl std::fmt::Binary for AlphaMultiset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl std::fmt::Debug for AlphaMultiset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..26 {
            for b in 0..MAX_CHAR_REP {
                if self.0.get(i * MAX_CHAR_REP + b).unwrap() {
                    write!(f, "{}", (A + i) as u8 as char)?;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for AlphaMultiset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_eq() {
        assert_eq!(
            AlphaMultiset::from("bcbcaz"),
            AlphaMultiset::from(&[
                1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );
    }

    #[test]
    #[should_panic(expected = "Can only handle 9 repetitions of chars, but 'a' occurs 12 times")]
    fn test_max_char_reps() {
        AlphaMultiset::from("aaabaaabaaabaaa");
    }

    #[test]
    #[should_panic(expected = "Can only handle lowercase ASCII alpha chars, but got '-'")]
    fn test_only_ascii() {
        AlphaMultiset::from("-");
    }

    #[test]
    fn test_display() {
        assert_eq!(AlphaMultiset::from("zmazzoa").to_string(), "aamozzz")
    }

    #[test]
    fn test_abcdefg_display() {
        assert_eq!(AlphaMultiset::from("abcdefg").to_string(), "abcdefg")
    }

    #[test]
    fn test_abc_subset_ab() {
        assert!(AlphaMultiset::from("abc").has_subset(&AlphaMultiset::from("ab")))
    }

    #[test]
    fn test_abd_not_subset_abc() {
        assert!(!AlphaMultiset::from("abd").has_subset(&AlphaMultiset::from("abc")))
    }
}
