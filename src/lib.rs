//! # AlgoX
//!
//! **AlgoX** is a high-performance, zero-overhead algorithmic foundation library written in pure Rust.
//! It offers **30 algorithm families** with standardized traits and production-ready implementations:
//!
//! - **Tree**: `SegmentTree`, `FenwickTree`, `AvlTree`, `RedBlackTree`, `BPlusTree`, `Trie`
//! - **Graph**: `BFS`, `DFS`, `Dijkstra`, `BellmanFord`, `FloydWarshall`, `TarjanSCC`, `Kruskal`, `Prim`, `Bridge`, `ConnectedComponents`, `TopologicalSort`
//! - **Dynamic Programming**: `Knapsack01`, `LCS`, `LIS`, `CoinChange`, `EditDistance`, `MatrixChain`
//! - **Network Flow**: `EdmondsKarp`, `BipartiteMatching`
//! - **Geometry**: `ConvexHull`, `ClosestPair`, `LineGeometry`, `Point`
//! - **String**: `ZAlgorithm`, `AhoCorasick`, `RabinKarp`, `Levenshtein`
//! - **Randomized**: `Shuffle`, `ReservoirSampling`
//! - **Math**: `Gcd`, `Sieve`, `ModArith`
//! - **Filtering**: `BloomFilter`, `CuckooFilter`, `HyperLogLog`, `CountMinSketch`, `QuotientFilter`, `ExactFilter`
//! - **Caching**: `LruCache`, `LfuCache`, `ArcCache`, `FifoCache`, `TtlCache`, `TwoQueueCache`
//! - **Compression**: `Gzip`, `Deflate`, `Brotli`, `Lz4`, `Zstd`
//! - **Cryptography**: AES (CBC/GCM), ChaCha20Poly1305, Fernet, RSA, ECDSA, Ed25519, X25519
//! - **Hashing**: MD5, SHA-1, SHA-2, SHA-3, BLAKE2, BLAKE3, CRC32, FNV-1a, xxHash, Argon2, bcrypt, scrypt, PBKDF2
//! - **Sorting**: `QuickSort`, `MergeSort`, `TimSort`, `HeapSort`, `RadixSort`, `CountingSort`, `InsertionSort`, `SelectionSort`, `BubbleSort`
//! - **Search**: `BinarySearch`, `ExponentialSearch`, `KmpSearch`, `LinearSearch`
//! - **Rate Limiting**: `TokenBucket`, `LeakyBucket`, `SlidingWindow`
//! - **Load Balancing**: `RoundRobin`, `WeightedRoundRobin`, `LeastConnections`
//! - **Backtracking**: `NQueens`, `SudokuSolver`, `Permutations`
//! - **Greedy**: `FractionalKnapsack`, `HuffmanCoding`
//! - **Spatial**: `KdTree`, `QuadTree`
//! - **Matrix**: `Matrix`, `GaussianElimination`
//! - **Clustering**: `KMeans`, `KNearestNeighbors`
//! - **Sequence**: `NeedlemanWunsch`, `RunLengthEncoding`
//!
//! ## Safety
//!
//! AlgoX forbids unsafe code and operates with zero memory overhead.

#![deny(unsafe_code)]

pub mod abstraction;
pub mod backtracking;
pub mod bit;
pub mod buffer;
pub mod caching;
pub mod clustering;
pub mod compression;
pub mod crypto;
pub mod disjoint_set;
pub mod dynamic_programming;
pub mod encoding;
pub mod filtering;
pub mod geometry;
pub mod graph;
pub mod greedy;
pub mod hashing;
pub mod heap;
pub mod indexing;
pub mod load_balancing;
pub mod math;
pub mod matrix;
pub mod network_flow;
pub mod randomized;
pub mod rate_limit;
pub mod scheduling;
pub mod search;
pub mod sequence;
pub mod sorting;
pub mod spatial;
pub mod string;
pub mod tree;

pub use abstraction::AlgorithmTrait;
pub use backtracking::{BacktrackingAlgorithmTrait, NQueens, Permutations, SudokuSolver};
pub use buffer::{BufferAlgorithmTrait, CircularBuffer, RingBuffer};
pub use caching::{
    ArcCache, CacheAlgorithmTrait, FifoCache, LfuCache, LruCache, TtlCache, TwoQueueCache,
};
pub use clustering::{ClusteringAlgorithmTrait, KMeans, KNearestNeighbors};
pub use compression::{Brotli, CompressionAlgorithmTrait, Deflate, Gzip, Lz4, Zstd};
pub use crypto::{
    Aes128Cbc, Aes128Gcm, Aes256Cbc, Aes256Gcm, CbcAlgorithmTrait, CipherAlgorithmTrait,
    ChaCha20Poly1305, CryptoAlgorithmTrait, EcdsaAlgorithmTrait, EcdsaP256, EcdsaP384, Ed25519,
    Fernet, KeyExchangeAlgorithmTrait, Rsa2048, Rsa3072, Rsa4096, RsaOaep, RsaPss,
    SignatureAlgorithmTrait, X25519,
};
pub use disjoint_set::{DisjointSetAlgorithmTrait, DisjointSetRank, DisjointSetSize};
pub use encoding::{Base64, Base64Url, EncodingAlgorithmTrait, Hex, UrlPercent};
pub use filtering::{
    BloomFilter, CountMinSketch, CuckooFilter, ExactFilter, FilterAlgorithmTrait, HyperLogLog,
    ProbabilisticFilterAlgorithmTrait, QuotientFilter,
};
pub use geometry::{ClosestPair, ConvexHull, GeometryAlgorithmTrait, LineGeometry, Point};
pub use graph::{
    BellmanFord, BFS, Bridge, ConnectedComponents, DFS, Dijkstra, FloydWarshall, GraphAlgorithmTrait,
    GraphTraversalAlgorithmTrait, Kruskal, Prim, ShortestPathAlgorithmTrait, TarjanSCC, TopologicalSort,
};
pub use greedy::{FractionalKnapsack, GreedyAlgorithmTrait, HuffmanCoding, Item};
pub use hashing::{
    blake2b, blake2s, blake3, crc32, fnv1a_32, fnv1a_64, md5, sha1, sha224, sha256, sha384,
    sha512, sha3_224, sha3_256, sha3_384, sha3_512, xxhash3, xxhash32, xxhash64, Argon2, Bcrypt,
    Blake2b, Blake2s, Blake3, Crc32, Fnv1a32, Fnv1a64, HashingAlgorithmTrait, Md5, Pbkdf2, Scrypt,
    Sha1, Sha224, Sha256, Sha384, Sha512, Sha3_224, Sha3_256, Sha3_384, Sha3_512, XxHash3, XxHash32,
    XxHash64,
};
pub use heap::{BinaryMaxHeap, BinaryMinHeap, HeapAlgorithmTrait};
pub use indexing::{BTreeIndex, HashIndex, IndexingAlgorithmTrait};
pub use load_balancing::{
    LeastConnections, LoadBalancingAlgorithmTrait, RoundRobin, WeightedRoundRobin,
};
pub use math::{Gcd, MathAlgorithmTrait, ModArith, Sieve};
pub use matrix::{GaussianElimination, Matrix, MatrixAlgorithmTrait};
pub use network_flow::{BipartiteMatching, EdmondsKarp, NetworkFlowAlgorithmTrait};
pub use randomized::{RandomizedAlgorithmTrait, ReservoirSampling, Shuffle};
pub use rate_limit::{LeakyBucket, RateLimitAlgorithmTrait, SlidingWindow, TokenBucket};
pub use scheduling::{ActivitySelection, Job, SchedulingAlgorithmTrait, WeightedJobScheduling};
pub use search::{
    BinarySearch, ExponentialSearch, KmpSearch, LinearSearch, SearchAlgorithmTrait,
};
pub use sequence::{NeedlemanWunsch, RunLengthEncoding, SequenceAlgorithmTrait};
pub use sorting::{
    BubbleSort, CountingSort, HeapSort, InsertionSort, MergeSort, QuickSort, RadixSort,
    SelectionSort, SortingAlgorithmTrait, TimSort,
};
pub use spatial::{BoundingBox, KdTree, Point2D, QuadTree, SpatialAlgorithmTrait};
pub use string::{AhoCorasick, Levenshtein, RabinKarp, StringAlgorithmTrait, ZAlgorithm};
pub use tree::{AvlTree, BPlusTree, FenwickTree, RedBlackTree, SegmentTree, TreeAlgorithmTrait, Trie};
pub use bit::{BitAlgorithmTrait, BitOps};
pub use dynamic_programming::{
    CoinChange, DynamicProgrammingTrait, EditDistance, Knapsack01, LCS, LIS, MatrixChain,
};
