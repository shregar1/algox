pub mod abstraction;
pub mod btree;
pub mod hash;

pub use abstraction::IndexingAlgorithmTrait;
pub use btree::BTreeIndex;
pub use hash::HashIndex;
