use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use wordscapes_cheater::*;

const DAG_FILENAME: &'static str = "dag.bin";

fn main() {
    let use_small = match std::env::args().nth(1) {
        Some(s) if s == "small" => true,
        Some(s) if s == "large" => false,
        _ => panic!("Usage: {} <small/large>\n\n\tsmall - use small dictionary\n\tlarge - use large dictionary", std::env::args().nth(0).unwrap())
    };
    let dict_path = if use_small { "wordlist.txt" } else { "wordlist_large.txt" };

    let cheater = WordscapesLookup::from_wordlist(dict_path);
    println!("Constructed wordlist DAG");

    let binarr = bincode::serialize(&cheater).expect("Unable to serialize DAG");
    println!("Serialized DAG to Vec<u8>");

    let path = &Path::new("src").join(DAG_FILENAME);
    let file = File::create(path)
        .unwrap_or_else(|_| panic!("Unable to create DAG file '{}'", path.display()));
    let mut writer = BufWriter::new(file);

    writer
        .write_all(&*binarr)
        .unwrap_or_else(|_| panic!("Couldn't write bytes to '{}'", path.display()));
    println!("Serialized Vec<u8> to '{}'", path.display());
}
