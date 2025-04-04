# LRU Cache Implementations in Rust

This repository contains different implementations of Least Recently Used (LRU) cache replacement algorithms in Rust. Currently implemented:

## 1. Square Matrix LRU Implementation

The Square Matrix LRU implementation uses a novel approach to track the least recently used items using a binary matrix. This implementation is particularly interesting for hardware implementations due to its simple update mechanism and deterministic behavior.

### How It Works

The implementation maintains an N×N matrix where N is the number of ways in the cache. The matrix is used to track the relative order of accesses between different ways:

1. **Matrix Structure**:
   - Each row corresponds to a cache way
   - Each column also corresponds to a cache way
   - Matrix elements are binary (0 or 1)

2. **Update Rules**:
   - When a way is accessed:
     - The entire row corresponding to that way is set to 1
     - The entire column corresponding to that way is set to 0
   - This creates a unique pattern where:
     - More 1s in a row indicates more recent access
     - A row with all 0s indicates the least recently used way

3. **LRU Detection**:
   - The least recently used way is identified by finding the row that contains all zeros
   - The number of 1s in a row indicates how recently that way was accessed relative to others

### Usage Example

```rust
use lru::square_matrix::SquareMatrixLRU;

// Create a new 4-way LRU cache
let mut lru = SquareMatrixLRU::new(4);

// Access way 0
lru.access(0);

// Access way 1
lru.access(1);

// Find the least recently used way
let lru_way = lru.find_lru();

// Get usage count for a way (number of 1s in its row)
let usage = lru.get_usage_count(0);
```

### API Reference

- `new(size: usize) -> Self`: Creates a new square matrix LRU of given size
- `access(&mut self, way: usize)`: Records an access to the specified way
- `find_lru(&self) -> usize`: Returns the index of the least recently used way
- `get_usage_count(&self, way: usize) -> usize`: Returns the number of 1s in the specified way's row

### Properties

1. **Space Complexity**: O(N²) where N is the number of ways
2. **Time Complexity**:
   - Access: O(N)
   - LRU lookup: O(N)
   - Usage count: O(N)

### Advantages

1. Simple and deterministic behavior
2. Hardware-friendly implementation
3. No complex data structures needed
4. Easy to verify and debug

### Limitations

1. Space complexity is quadratic
2. All operations are O(N) complexity
3. May not be the most efficient for software implementations

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the example
cargo run
```

## Requirements

- Rust 2021 edition or later
- Cargo package manager

## License

This project is available under the MIT license.

---

More LRU implementations coming soon:
- Counter-based LRU
- Tree-based LRU
- Timestamp-based LRU 