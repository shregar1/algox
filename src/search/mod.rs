pub mod abstraction;
pub mod binary;
pub mod exponential;
pub mod kmp;
pub mod linear;

pub use abstraction::SearchAlgorithmTrait;
pub use binary::BinarySearch;
pub use exponential::ExponentialSearch;
pub use kmp::KmpSearch;
pub use linear::LinearSearch;
