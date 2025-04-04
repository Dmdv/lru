mod square_matrix;

use square_matrix::SquareMatrixLRU;

fn main() {
    // Create a 4-way LRU cache
    let mut lru = SquareMatrixLRU::new(4);
    
    // Simulate some accesses
    println!("Initial state:");
    for i in 0..4 {
        println!("Way {} usage count: {}", i, lru.get_usage_count(i));
    }
    
    // Access way 0
    lru.access(0);
    println!("\nAfter accessing way 0:");
    for i in 0..4 {
        println!("Way {} usage count: {}", i, lru.get_usage_count(i));
    }
    
    // Access way 1
    lru.access(1);
    println!("\nAfter accessing way 1:");
    for i in 0..4 {
        println!("Way {} usage count: {}", i, lru.get_usage_count(i));
    }
    
    // Find the LRU way
    println!("\nLRU way: {}", lru.find_lru());
}
