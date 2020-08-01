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
    println!("Words that can be made from '{}':", letters);
    for word in cheater.lookup(&*letters) {
        println!("\t{}", word);
    }
    println!("Lookup took {:?}", start.elapsed());
}
