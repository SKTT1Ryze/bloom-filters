//! Multi Bloom Filter Implementation
use proptest::strategy::Filter;

use crate::{BloomFilter, MultiBloomFilter};

pub struct DefaultMultiBloomFilter<BF: BloomFilter, const N: usize> {
    bloom_filters: [BF; N]
}

impl<B: BloomFilter, const N: usize> DefaultMultiBloomFilter<B, N> {
    pub fn new(filters: [B; N]) -> Self {
        Self {
            bloom_filters: filters
        }
    }
}
impl<B: BloomFilter, const N: usize> IntoIterator for DefaultMultiBloomFilter<B, N> {
    type Item = B;
    type IntoIter = std::array::IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.bloom_filters)
    }
}


impl<B: BloomFilter, const N: usize> MultiBloomFilter for DefaultMultiBloomFilter<B, N> {
    type BF = B;
    type BI = std::array::IntoIter<B, N>;
    fn bloom_filter(self) -> Self::BI {
        self.into_iter()
    }
}

#[test]
fn default_multi_bloom_filter_test() {
    use crate::StableBloomFilter;
    use crate::hash::DefaultBuildHashKernels;
    use crate::buckets::compute_word_num;
    use rand::random;
    use std::collections::hash_map::RandomState;
    use crate::filter;

    let filtes = [
        filter!(72, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new())),
        filter!(72, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new())),
        filter!(72, 3, 0.03, DefaultBuildHashKernels::new(random(), RandomState::new()))
        ];
    let multi_filter = DefaultMultiBloomFilter::new(filtes);
    let items = [vec![1; 10], vec![1; 10], vec![1; 10]];
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
}