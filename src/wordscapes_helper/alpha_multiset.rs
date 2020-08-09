use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const NUM_BLOCKS: usize = 4;
const BLOCK_SIZE: usize = 64;
const A: usize = 'a' as usize;
const Z: usize = 'z' as usize;
const MAX_CHAR_REP: usize = NUM_BLOCKS * BLOCK_SIZE / 26;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Serialize, Deserialize)]
pub struct BitArray([u64; NUM_BLOCKS]);

impl BitArray {
    /// Create BitArray with all ones
    pub fn new_ones() -> Self {
        Self([u64::MAX, u64::MAX, u64::MAX, u64::MAX])
    }

    /// Get specific bit, with no safety checks
    pub unsafe fn get_unchecked(&self, i: usize) -> bool {
        let b = i / BLOCK_SIZE;
        let n = i % BLOCK_SIZE;
        (self.0.get_unchecked(b) >> n) & 0b1 == 1
    }

    /// Get specific bit
    pub fn get(&self, i: usize) -> Option<bool> {
        if i >= NUM_BLOCKS * BLOCK_SIZE {
            None
        } else {
            Some(unsafe { self.get_unchecked(i) })
        }
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

    pub fn is_empty(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] == 0 && self.0[3] == 0
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

    /// Return the least index which is set
    pub fn least_set_index(&self) -> usize {
        let bs = |a: u64| a.trailing_zeros() as usize;

        match (bs(self.0[0]), bs(self.0[1]), bs(self.0[2]), bs(self.0[3])) {
            (BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE, a) => 3 * BLOCK_SIZE + a,
            (BLOCK_SIZE, BLOCK_SIZE, a, _) => 2 * BLOCK_SIZE + a,
            (BLOCK_SIZE, a, _, _) => BLOCK_SIZE + a,
            (a, _, _, _) => a,
        }
    }

    /// Return the greatest index which is set
    pub fn greatest_set_index(&self) -> usize {
        let bs = |a: u64| a.leading_zeros() as usize;

        match (bs(self.0[0]), bs(self.0[1]), bs(self.0[2]), bs(self.0[3])) {
            (a, BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE) => 3 * BLOCK_SIZE + a,
            (_, a, BLOCK_SIZE, BLOCK_SIZE) => 2 * BLOCK_SIZE + a,
            (_, _, a, BLOCK_SIZE) => BLOCK_SIZE + a,
            (_, _, _, a) => a,
        }
    }

    pub fn intersection(&self, other: &Self) -> BitArray {
        BitArray([
            self.0[0] & other.0[0],
            self.0[1] & other.0[1],
            self.0[2] & other.0[2],
            self.0[3] & other.0[3],
        ])
    }

    pub fn simple_union(&self, other: &Self) -> BitArray {
        BitArray([
            self.0[0] | other.0[0],
            self.0[1] | other.0[1],
            self.0[2] | other.0[2],
            self.0[3] | other.0[3],
        ])
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

#[derive(Eq, PartialEq, Clone, Hash, Default, Serialize, Deserialize)]
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

    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Check if `self` has `other` as a subset (not strict)
    pub fn has_subset(&self, other: &Self) -> bool {
        self.0.has_subset(&other.0)
    }

    /// Check if `self` is disjoint from `other`
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    /// Get the least alpha value which is in the set, in lowercase
    pub fn least_entry(&self) -> u8 {
        (self.0.least_set_index() / MAX_CHAR_REP) as u8 + 'a' as u8
    }

    /// Remove a single entry from this multiset
    /// Returns true if an entry was removed, false otherwise
    pub fn remove_entry(&mut self, ch: u8) -> bool {
        let ci = (ch as usize - A) * MAX_CHAR_REP;

        // could this be faster?
        for i in (0..MAX_CHAR_REP).rev() {
            if self.0.get(ci + i).unwrap() {
                self.0.set(ci + i, false);
                return true;
            }
        }
        return false;
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0))
    }

    pub fn simple_union(&self, other: &Self) -> Self {
        Self(self.0.simple_union(&other.0))
    }

    /// Compute the multiset of `self - other`
    pub fn difference(&self, other: &Self) -> Self {
        let mut dif = Self::default();

        for a in 0..26 {
            let mut di = 0;
            for i in (0..MAX_CHAR_REP).rev() {
                match (
                    self.0.get(a * MAX_CHAR_REP + i),
                    other.0.get(a * MAX_CHAR_REP + i),
                ) {
                    (Some(true), Some(false)) => {
                        dif.0.set(a * MAX_CHAR_REP + di, true);
                        di += 1;
                    }
                    (Some(true), Some(true)) | (Some(false), Some(true)) => break,
                    _ => {}
                }
            }
        }

        return dif;
    }

    pub fn char_counts(&self) -> [u8; 26] {
        let mut counts: [u8; 26] = Default::default();

        for i in 0..26 {
            for b in 0..MAX_CHAR_REP {
                if self.0.get(i * MAX_CHAR_REP + b).unwrap() {
                    counts[i] += 1;
                } else {
                    break;
                }
            }
        }

        return counts;
    }
}

impl PartialOrd for AlphaMultiset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.has_subset(other) {
            Some(Ordering::Greater)
        } else if other.has_subset(self) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl AsRef<BitArray> for AlphaMultiset {
    fn as_ref(&self) -> &BitArray {
        &self.0
    }
}

impl AsMut<BitArray> for AlphaMultiset {
    fn as_mut(&mut self) -> &mut BitArray {
        &mut self.0
    }
}

impl std::convert::Into<BitArray> for AlphaMultiset {
    fn into(self) -> BitArray {
        self.0
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

    #[test]
    fn test_least_entry() {
        assert_eq!(AlphaMultiset::from("maaz").least_entry(), b'a');
        assert_ne!(AlphaMultiset::from("mz").least_entry(), b'z');
    }

    #[test]
    fn test_remove_least() {
        let mut set = AlphaMultiset::from("maazm");
        assert_eq!(set.to_string(), "aammz");
        assert_eq!(set.least_entry(), b'a');

        set.remove_entry(set.least_entry());
        assert_eq!(set.to_string(), "ammz");
        assert_eq!(set.least_entry(), b'a');

        set.remove_entry(set.least_entry());
        assert_eq!(set.to_string(), "mmz");
        assert_eq!(set.least_entry(), b'm');

        set.remove_entry(set.least_entry());
        assert_eq!(set.to_string(), "mz");
        assert_eq!(set.least_entry(), b'm');
    }

    #[test]
    fn test_difference() {
        assert_eq!(
            AlphaMultiset::from("maaaz").difference(&AlphaMultiset::from("zaamim")),
            AlphaMultiset::from("a")
        );
    }
}
