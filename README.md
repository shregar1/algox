# AlgoX ⚡

[![Crates.io](https://img.shields.io/crates/v/algox.svg)](https://crates.io/crates/algox)
[![Documentation](https://docs.rs/algox/badge.svg)](https://docs.rs/algox)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**AlgoX** is a high-performance, zero-overhead algorithmic foundation library in pure Rust, offering **31 algorithm families** with standardized traits and pure-Rust implementations.

---

## 📦 Features & Algorithm Families

| Family | Submodules & Algorithms |
|--------|------------------------|
| **Sharding & Partitioning** | `ConsistentSharding` (Virtual Nodes), `RendezvousSharding` (HRW), `JumpConsistentSharding` (Google $O(\ln N)$), `MaglevSharding` (Google $O(1)$ Table), `RangeSharding`, `GeoSharding` (Geohash routing), `ListSharding`, `CompositeSharding` (Co-locating data), `DirectorySharding` (Dynamic map lookup) |
| **Hashing** | MD5, SHA-1, SHA-2, SHA-3, BLAKE2b/s, BLAKE3, CRC32, FNV-1a, xxHash, Argon2, bcrypt, scrypt, PBKDF2, `Geohash` (Spatial Hashing) |
| **Trees** | `SegmentTree`, `FenwickTree` (BIT), `AvlTree`, `RedBlackTree`, `BPlusTree`, `Trie` |
| **Graphs** | `BFS`, `DFS`, `Dijkstra`, `BellmanFord`, `FloydWarshall`, `TarjanSCC`, `Kruskal`, `Prim`, `Bridge`, `ConnectedComponents`, `TopologicalSort` |
| **Dynamic Programming** | `Knapsack01`, `LCS`, `LIS`, `CoinChange`, `EditDistance`, `MatrixChain` |
| **Network Flow** | `EdmondsKarp` (Max Flow), `BipartiteMatching` |
| **Computational Geometry** | `ConvexHull` (Graham Scan), `ClosestPair` ($O(n \log n)$), `LineGeometry` (Ray Casting & Segment Intersect) |
| **String Matching** | `ZAlgorithm`, `AhoCorasick`, `RabinKarp`, `Levenshtein` |
| **Randomized** | `Shuffle` (Fisher-Yates), `ReservoirSampling` |
| **Math & Number Theory** | `Gcd`/`LCM`/Extended Euclidean, `Sieve` of Eratosthenes, `ModArith` (Fast Modular Exponentiation & Inverse) |
| **Scheduling** | `ActivitySelection` (Greedy), `WeightedJobScheduling` (DP) |
| **Bit Manipulation** | `BitOps` (Popcount, Hamming Distance, Bit Reversal, Next Power of 2) |
| **Filtering** | `BloomFilter`, `CuckooFilter`, `HyperLogLog`, `CountMinSketch`, `QuotientFilter`, `ExactFilter` |
| **Disjoint Set** | `DisjointSetRank`, `DisjointSetSize` |
| **Buffers** | `CircularBuffer`, `RingBuffer` |
| **Caching** | `LruCache`, `LfuCache`, `ArcCache`, `FifoCache`, `TtlCache`, `TwoQueueCache` |
| **Compression** | `Gzip`, `Deflate`, `Brotli`, `Lz4`, `Zstd` |
| **Cryptography** | AES (CBC/GCM), ChaCha20Poly1305, Fernet, RSA, ECDSA (P-256/P-384), Ed25519, X25519 |
| **Encoding** | `Base64`, `Base64Url`, `Hex`, `UrlPercent` |
| **Heap** | `BinaryMinHeap`, `BinaryMaxHeap` |
| **Indexing** | `BTreeIndex`, `HashIndex` |
| **Load Balancing** | `RoundRobin`, `WeightedRoundRobin`, `LeastConnections` |
| **Rate Limiting** | `TokenBucket`, `LeakyBucket`, `SlidingWindow` |
| **Search** | `LinearSearch`, `BinarySearch`, `ExponentialSearch`, `KmpSearch` |
| **Sorting** | `BubbleSort`, `InsertionSort`, `SelectionSort`, `MergeSort`, `QuickSort`, `HeapSort`, `RadixSort`, `CountingSort`, `TimSort` |
| **Backtracking** | `NQueens`, `SudokuSolver`, `Permutations` |
| **Greedy** | `FractionalKnapsack`, `HuffmanCoding` |
| **Spatial** | `KdTree` (2D Nearest Neighbor), `QuadTree` (2D Range Querying) |
| **Matrix** | `Matrix` (Multiplication & Transpose), `GaussianElimination` (Linear Systems) |
| **Clustering & ML** | `KMeans` (Clustering), `KNearestNeighbors` (KNN Classification) |
| **Sequence** | `NeedlemanWunsch` (Global Alignment), `RunLengthEncoding` (RLE Compression) |

---

## 🚀 Quick Start

Add `algox` to your `Cargo.toml`:

```toml
[dependencies]
algox = "0.1"
```

### Example: Geo-Sharding Datacenter Router

```rust
use algox::{GeoSharding, Geohash};

fn main() {
    let mut sharder = GeoSharding::new(5);
    sharder.set_fallback("global-us-east-datacenter");

    // Register Europe prefix "ezs" to European shard cluster
    sharder.register_region("ezs", "shard-eu-west");

    // Route coordinates (42.6, -5.6) -> geohash "ezs42" -> routes to "shard-eu-west"
    let target_shard = sharder.route_coords(42.6, -5.6).unwrap();
    assert_eq!(target_shard, "shard-eu-west");
}
```

---

## 📜 License

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.
