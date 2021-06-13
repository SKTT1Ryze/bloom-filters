use bloom_filters::{BloomFilter, DefaultBuildHashKernels, StableBloomFilter, DefaultMultiBloomFilter};
use bloom_filters::buckets::compute_word_num;
use bloom_filters::filter;
use criterion::{criterion_group, criterion_main, Criterion, Fun};
use rand::distributions::Standard;
use rand::{random, thread_rng, Rng};
use std::collections::hash_map::RandomState;

fn bench(c: &mut Criterion) {
    let multi = Fun::new("multi", |b, fp_rate| {
        let items0: Vec<usize> = thread_rng().sample_iter(&Standard).take(7).collect();
        let items1: Vec<usize> = thread_rng().sample_iter(&Standard).take(7).collect();
        let items2: Vec<usize> = thread_rng().sample_iter(&Standard).take(7).collect();
        let items = [items0, items1, items2];
        b.iter(|| {
            let filtes = [
                filter!(73, 3, *fp_rate, DefaultBuildHashKernels::new(random(), RandomState::new())),
                filter!(73, 3, *fp_rate, DefaultBuildHashKernels::new(random(), RandomState::new())),
                filter!(73, 3, *fp_rate, DefaultBuildHashKernels::new(random(), RandomState::new()))
            ];
            let multi_filter = DefaultMultiBloomFilter::new(filtes);
            let iter: Vec<_> = multi_filter
                .into_iter()
                .zip(items.iter())
                .map(|(mut f, i)| {
                    i.iter().for_each(|item| f.insert(item));
                    f
                })
                .collect();
            let ret = iter
                .iter()
                .zip(items.iter())
                .all(|(f, i)| {
                    i.iter().all(|item| f.contains(item))
                });
            assert!(ret);
        });
    });
    c.bench_functions("multi", vec![multi], 0.03);
}

criterion_group!(benches, bench);
criterion_main!(benches);