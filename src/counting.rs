use buckets::Buckets;
use std::hash::{BuildHasher, Hash};
use {BloomFilter, DoubleHashing, HashKernals, RemovableBloomFilter};

pub struct Filter<BH> {
    buckets: Buckets,                // filter data
    hash_kernals: DoubleHashing<BH>, // a hash function builder
}

impl<BH: BuildHasher> Filter<BH> {
    /// Create a new bloom filter structure.
    /// items_count is an estimation of the maximum number of items to store.
    /// bucket_size is the specified number of bits
    /// fp_rate is the wanted rate of false positives, in ]0.0, 1.0[
    pub fn new(items_count: usize, bucket_size: u8, fp_rate: f64, build_hasher: BH) -> Self {
        let buckets = Buckets::with_fp_rate(items_count, fp_rate, bucket_size);
        let hash_kernals = DoubleHashing::with_fp_rate(fp_rate, buckets.len(), build_hasher);
        Self { buckets, hash_kernals }
    }
}

impl<BH: BuildHasher> BloomFilter for Filter<BH> {
    fn insert<T: Hash>(&mut self, item: &T) {
        self.hash_kernals.hash_iter(item).for_each(|i| self.buckets.increment(i, 1))
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        self.hash_kernals.hash_iter(item).all(|i| self.buckets.get(i) > 0)
    }

    fn reset(&mut self) {
        self.buckets.reset()
    }
}

impl<BH: BuildHasher> RemovableBloomFilter for Filter<BH> {
    fn remove<T: Hash>(&mut self, item: &T) {
        self.hash_kernals.hash_iter(item).for_each(|i| self.buckets.increment(i, -1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::{thread_rng, Rng};
    use std::collections::hash_map::RandomState;

    #[test]
    fn contains() {
        let mut filter = Filter::new(100, 4, 0.03, RandomState::new());
        let items: Vec<usize> = thread_rng().sample_iter(&Standard).take(16).collect();
        assert!(items.iter().all(|i| !filter.contains(i)));
        items.iter().for_each(|i| filter.insert(i));
        assert!(items.iter().all(|i| filter.contains(i)));
    }

    #[test]
    fn remove() {
        let mut filter = Filter::new(100, 4, 0.03, RandomState::new());
        let item: usize = thread_rng().gen();
        filter.insert(&item);
        filter.remove(&item);
        assert!(!filter.contains(&item));
    }
}
