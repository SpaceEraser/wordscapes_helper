use std::collections::HashMap;

fn main() {
    let words =
        std::fs::read_to_string("wordlist_large.txt").expect("Unable to find 'wordlist_large.txt'");
    for w in words.trim().lines().map(str::trim) {
        let mut m = HashMap::<char, u8>::new();
        for c in w.chars() {
            *m.entry(c).or_default() += 1;
        }
        for (k, v) in m {
            const LIM: u8 = 9;
            if v > LIM {
                panic!(
                    "Found word with more than {} of the same letter: {}*{} in {}",
                    LIM, v, k, w
                );
            }
        }
    }
}
