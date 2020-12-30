use criterion::{Criterion, black_box, criterion_group, criterion_main};
use trie_benchmark::naive_trie::NaiveTrie;
use anyhow::Result;

pub fn trie_insertion(c: &mut Criterion) {
    let data = match load_data() {
        Err(err) => {
            eprintln!("fail to load data: {:?}", err);
            return
        }
        Ok(data) => data,
    };
    c.bench_function("naive trie insertion", |b| b.iter(|| {
        let mut trie = NaiveTrie::new();
        for word in &data {
            trie.insert(word.as_bytes());
        }
    }));
}

fn load_data() -> Result<Vec<String>> {
    let text = std::fs::read_to_string("./data/bible.txt")?;
    Ok(text.split_ascii_whitespace().map(|s| s.to_string()).collect())
}

criterion_group!(benches, trie_insertion);
criterion_main!(benches);