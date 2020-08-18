use wordscapes_helper::*;

fn main() {
    let letters = if let Some(letters) = std::env::args().nth(1) {
        letters
    } else {
        println!(
            "Usage: {} <characters to look up>",
            std::env::args().nth(0).unwrap()
        );
        std::process::exit(1)
    };

    let start = std::time::Instant::now();
    let helper = DAGSearcher::default();
    // let helper = TrieSearcher::from_wordlist("wordlist_large.txt");
    // let helper = SimpleSearcher::from_wordlist("wordlist_large.txt");
    // let helper = ExpSearcher::from_wordlist("wordlist_large.txt");
    let elapsed = start.elapsed();
    println!("Setup took {:?}", elapsed);

    let start = std::time::Instant::now();
    let mut words = helper.lookup(&*letters);
    let elapsed = start.elapsed();

    words.sort_unstable_by_key(Word::frequency);

    println!("Words that can be made from '{}':", letters);
    for word in &words {
        println!("\t{}", word);
    }
    println!("Found {} items", words.len());
    println!("Lookup took {:?}", elapsed);
}
