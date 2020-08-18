use criterion::{criterion_group, criterion_main, Criterion};

use wordscapes_helper::*;

fn bench_dag(c: &mut Criterion) {
    let searcher = DAGSearcher::default();

    c.bench_function("dag `abc`", |b| b.iter(|| searcher.lookup("abc")));
    c.bench_function("dag `abcdef`", |b| b.iter(|| searcher.lookup("abcdef")));
    c.bench_function("dag `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
    c.bench_function("dag `abcdefghijklmnopqrstuvwx`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
}

fn bench_trie(c: &mut Criterion) {
    let searcher = TrieSearcher::default();

    c.bench_function("trie `abc`", |b| b.iter(|| searcher.lookup("abc")));
    c.bench_function("trie `abcdef`", |b| b.iter(|| searcher.lookup("abcdef")));
    c.bench_function("trie `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
    c.bench_function("trie `abcdefghijklmnopqrstuvwx`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
}

fn bench_exp(c: &mut Criterion) {
    let searcher = ExpSearcher::default();

    c.bench_function("exp `abc`", |b| b.iter(|| searcher.lookup("abc")));
    c.bench_function("exp `abcdef`", |b| b.iter(|| searcher.lookup("abcdef")));
    c.bench_function("exp `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
}

fn bench_simple(c: &mut Criterion) {
    let searcher = SimpleSearcher::default();

    c.bench_function("simple `abc`", |b| b.iter(|| searcher.lookup("abc")));
    c.bench_function("simple `abcdef`", |b| b.iter(|| searcher.lookup("abcdef")));
    c.bench_function("simple `abcdefghijkl`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
    c.bench_function("simple `abcdefghijklmnopqrstuvwx`", |b| {
        b.iter(|| searcher.lookup("abcdefghijkl"))
    });
}

criterion_group!(benches, bench_dag, bench_trie, bench_exp, bench_simple);
criterion_main!(benches);
