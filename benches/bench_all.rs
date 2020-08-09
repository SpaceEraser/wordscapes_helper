use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wordscapes_helper::*;

fn bench_dag(c: &mut Criterion) {
    let searcher = DAGSearcher::from_embedded_wordlist();
    
    c.bench_function("dag `abc`", |b| {
        b.iter(|| searcher.lookup(black_box("abc")))
    });
    c.bench_function("dag `abcdef`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdef")))
    });
    c.bench_function("dag `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdefghijkl")))
    });
}

fn bench_trie(c: &mut Criterion) {
    let searcher = TrieSearcher::from_wordlist("wordlist_large.txt");
    
    c.bench_function("simple `abc`", |b| {
        b.iter(|| searcher.lookup(black_box("abc")))
    });
    c.bench_function("simple `abcdef`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdef")))
    });
    c.bench_function("simple `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdefghijkl")))
    });
}

fn bench_simple(c: &mut Criterion) {
    let searcher = SimpleSearcher::from_wordlist("wordlist_large.txt");
    
    c.bench_function("simple `abc`", |b| {
        b.iter(|| searcher.lookup(black_box("abc")))
    });
    c.bench_function("simple `abcdef`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdef")))
    });
    c.bench_function("simple `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdefghijkl")))
    });
}

fn bench_exp(c: &mut Criterion) {
    let searcher = ExpSearcher::from_wordlist("wordlist_large.txt");
    
    c.bench_function("simple `abc`", |b| {
        b.iter(|| searcher.lookup(black_box("abc")))
    });
    c.bench_function("simple `abcdef`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdef")))
    });
    c.bench_function("simple `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup(black_box("abcdefghijkl")))
    });
}

criterion_group!(benches, bench_dag, bench_trie, bench_exp, bench_simple);
criterion_main!(benches);
