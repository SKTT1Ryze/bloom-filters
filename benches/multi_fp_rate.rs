use bloom_filters::{BloomFilter, DefaultBuildHashKernels, StableBloomFilter, DefaultMultiBloomFilter};
use bloom_filters::buckets::compute_word_num;
use bloom_filters::filter;
use criterion::{criterion_group, criterion_main, Criterion, Fun};
use rand::distributions::Standard;
use rand::{random, thread_rng, Rng};
use std::collections::hash_map::RandomState;

fn bench(c: &mut Criterion) {
    let filtes = [
        filter!(730, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new())),
        filter!(730, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new())),
        filter!(730, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new()))
    ];
    let multi_filter = DefaultMultiBloomFilter::new(filtes);
    let mut count = 0;
    let mut false_positives = 0;
    let mut iter: Vec<_> = multi_filter.into_iter().collect();
    while count < 100000 {
        let items0: Vec<usize> = thread_rng().sample_iter(&Standard).take(2).collect();
        let items1: Vec<usize> = thread_rng().sample_iter(&Standard).take(2).collect();
        let items2: Vec<usize> = thread_rng().sample_iter(&Standard).take(2).collect();
        let items = [items0, items1, items2];
        iter = iter
            .iter_mut()
            .zip(items.iter())
            .map(|(f, i)| {
                f.insert(&i[0]);
                f.clone()
            })
            .collect();
        
        iter = iter
            .iter_mut()
            .zip(items.iter())
            .map(|(f, i)| {
                if f.contains(&i[1]) { false_positives += 1; }
                f.clone()
            })
            .collect();
        
        count += 1;
    }
    
    println!("MultiBloomFilter false positives: {:?}", false_positives as f32 / 100000.0);
    let multi = Fun::new("multi", |b, _| b.iter(|| {}));
    let functions = vec![multi];
    c.bench_functions("multi false_positives_rate", functions, ());
}

criterion_group!(benches, bench);
criterion_main!(benches);