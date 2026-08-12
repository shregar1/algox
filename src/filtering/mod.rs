pub mod abstraction;
pub mod exact;
pub mod probabilistic;

pub use abstraction::FilterAlgorithmTrait;
pub use exact::ExactFilter;
pub use probabilistic::{
    BloomFilter, CountMinSketch, CuckooFilter, HyperLogLog, ProbabilisticFilterAlgorithmTrait,
    QuotientFilter,
};
