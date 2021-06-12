use bloom_filters::{BloomFilter, DefaultBuildHashKernels, StableBloomFilter};
use bloom_filters::buckets::compute_word_num;
use bloom_filters::filter;
use criterion::{criterion_group, criterion_main, Criterion, Fun};
use rand::distributions::Standard;
use rand::{random, thread_rng, Rng};
use std::collections::hash_map::RandomState;

// This is an empty bench, only print false positives rate
fn bench(c: &mut Criterion) {
    // item count: 100
    // bucket size: 2
    // fp rate: 0.03
    // bucket count = -100 * ln(0.03) / ln2 ^ 2 = 729.844
    let mut filter = filter!(730, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new()));
    let false_positives: usize = (0..100000)
        .filter(|_| {
            let items: Vec<usize> = thread_rng().sample_iter(&Standard).take(2).collect();
            filter.insert(&items[0]);
            filter.contains(&items[1])
        })
        .count();

    println!("StableBloomFilter false positives: {:?}", false_positives as f32 / 100000.0);


    let stable = Fun::new("stable", |b, _| b.iter(|| {}));
    let functions = vec![stable];
    c.bench_functions("false_positives_rate", functions, ());
}

criterion_group!(benches, bench);
criterion_main!(benches);
