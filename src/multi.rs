//! Multi Bloom Filter Implementation
use crate::{BloomFilter, MultiBloomFilter};

#[derive(Clone, Copy)]
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
    
}