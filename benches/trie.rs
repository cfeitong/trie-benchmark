use criterion::{Criterion, black_box, criterion_group, criterion_main};
use trie_benchmark::naive_trie::NaiveTrie;
use rand::prelude::*;

pub fn trie_insertion(c: &mut Criterion) {
    let data = load_bible_as_words();
    c.bench_function("naive trie insertion", |b| b.iter(|| {
        let mut trie = NaiveTrie::new();
        for word in &data {
            trie.insert(word.as_bytes());
        }
    }));
}

pub fn trie_lookup(c: &mut Criterion) {
    let mut data = load_bible_as_words();
    let mut rng = rand::thread_rng();
    let mut trie = NaiveTrie::new();
    for word in &data[0..data.len()/3] {
        trie.insert(word.as_bytes());
    }
    c.bench_function("naive trie lookup", |b| b.iter_with_setup(|| {
        data.shuffle(&mut rng);
        data.clone()
    }, |data| {
        for word in data {
            black_box(trie.contains(word.as_bytes()));
        }
    }));
}

fn load_bible_as_words() -> Vec<String> {
    let text = std::fs::read_to_string("./data/bible.txt").expect("fail to read bible");
    text.split_ascii_whitespace().map(|s| s.to_string()).collect()
}

criterion_group!(benches, trie_insertion, trie_lookup);
criterion_main!(benches);