use wordscapes_cheater::*;

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
    let cheater = WordscapesLookup::from_embedded_wordlist();

    let start = std::time::Instant::now();
    let words = cheater.lookup(&*letters);
    let elapsed = start.elapsed();
    
    println!("Words that can be made from '{}':", letters);
    for word in &words {
        println!("\t{}", word);
    }
    println!("Found {} items", words.len());
    println!("Lookup took {:?}", elapsed);
}
