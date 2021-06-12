use std::hash::Hash;

pub mod buckets;
pub mod hash;
pub mod stable;

pub use crate::hash::{BuildHashKernels, DefaultBuildHashKernels, DefaultBuildHasher, DefaultHashKernels, HashKernels};
pub use crate::stable::Filter as StableBloomFilter;

pub trait BloomFilter {
    fn insert<T: Hash>(&mut self, item: &T);
    fn contains<T: Hash>(&self, item: &T) -> bool;
    fn reset(&mut self);
}

pub trait RemovableBloomFilter {
    fn remove<T: Hash>(&mut self, item: &T);
}

pub trait UpdatableBloomFilter {
    /// Update filter internal buckets with `raw_data` via `BitOr` operation
    fn update(&mut self, raw_data: &[u8]);
}
