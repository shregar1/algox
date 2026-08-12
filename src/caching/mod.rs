pub mod abstraction;
pub mod arc;
pub mod fifo;
pub mod lfu;
pub mod lru;
pub mod ttl;
pub mod two_queue;

pub use abstraction::CacheAlgorithmTrait;
pub use arc::ArcCache;
pub use fifo::FifoCache;
pub use lfu::LfuCache;
pub use lru::LruCache;
pub use ttl::TtlCache;
pub use two_queue::TwoQueueCache;
