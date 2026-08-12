pub mod abstraction;
pub mod brotli;
pub mod deflate;
pub mod gzip;
pub mod lz4;
pub mod zstd;

pub use abstraction::CompressionAlgorithmTrait;
pub use brotli::Brotli;
pub use deflate::Deflate;
pub use gzip::Gzip;
pub use lz4::Lz4;
pub use zstd::Zstd;
