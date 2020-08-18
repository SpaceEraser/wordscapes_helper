use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use wordscapes_helper::*;

const DAG_FILENAME: &str = "dag.bin";

fn main() {
    let helper = DAGSearcher::from_embedded_wordlist();
    println!("Constructed wordlist DAG");

    let binarr = bincode::serialize(&helper).expect("Unable to serialize DAG");
    println!("Serialized DAG to Vec<u8>");

    let path = &Path::new("src").join("word_searcher").join(DAG_FILENAME);
    let file = File::create(path)
        .unwrap_or_else(|_| panic!("Unable to create DAG file '{}'", path.display()));
    let mut writer = BufWriter::new(file);

    writer
        .write_all(&*binarr)
        .unwrap_or_else(|_| panic!("Couldn't write bytes to '{}'", path.display()));
    println!("Serialized Vec<u8> to '{}'", path.display());
}
