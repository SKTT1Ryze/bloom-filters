use bloom_filters::{BloomFilter, DefaultBuildHashKernels, StableBloomFilter};
use bloom_filters::buckets::compute_word_num;
use criterion::{criterion_group, criterion_main, Criterion, Fun};
use rand::distributions::Standard;
use rand::{random, thread_rng, Rng};
use std::collections::hash_map::RandomState;

fn bench(c: &mut Criterion) {
    let stable = Fun::new("stable", |b, fp_rate| {
        // item count: 10
        // bucket size: 3
        // fp rate: 0.03
        // bucket count = -10 * ln(0.03) / ln2 ^ 2 = 72.9844
        let mut filter = StableBloomFilter::<_, {compute_word_num(73, 3)}, 73, 3>::new( *fp_rate, DefaultBuildHashKernels::new(random(), RandomState::new()));
        let items: Vec<usize> = thread_rng().sample_iter(&Standard).take(16).collect();
        b.iter(|| {
            items.iter().for_each(|i| {
                filter.insert(i);
            })
        })
    });
    let functions = vec![stable];
    c.bench_functions("insert", functions, 0.03);
}

criterion_group!(benches, bench);
criterion_main!(benches);
