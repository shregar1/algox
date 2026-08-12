pub mod abstraction;
pub mod needleman_wunsch;
pub mod rle;

pub use abstraction::SequenceAlgorithmTrait;
pub use needleman_wunsch::NeedlemanWunsch;
pub use rle::RunLengthEncoding;
