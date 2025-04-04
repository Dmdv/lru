use std::ops::{Index, IndexMut};

pub struct SquareMatrixLRU {
    matrix: Vec<Vec<bool>>,
    size: usize,
}

impl SquareMatrixLRU {
    pub fn new(size: usize) -> Self {
        SquareMatrixLRU {
            matrix: vec![vec![false; size]; size],
            size,
        }
    }

    pub fn access(&mut self, way: usize) {
        // Set entire row to 1
        for i in 0..self.size {
            self.matrix[way][i] = true;
        }
        
        // Set entire column to 0
        for i in 0..self.size {
            self.matrix[i][way] = false;
        }
    }

    pub fn find_lru(&self) -> usize {
        // Find the row with all zeros
        for way in 0..self.size {
            if self.matrix[way].iter().all(|&x| !x) {
                return way;
            }
        }
        // This should never happen if the matrix is properly maintained
        panic!("No LRU way found");
    }

    pub fn get_usage_count(&self, way: usize) -> usize {
        self.matrix[way].iter().filter(|&&x| x).count()
    }
}

impl Index<usize> for SquareMatrixLRU {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl IndexMut<usize> for SquareMatrixLRU {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_matrix_lru() {
        let mut lru = SquareMatrixLRU::new(4);
        
        // Access way 0
        lru.access(0);
        assert_eq!(lru.get_usage_count(0), 4);
        assert_eq!(lru.get_usage_count(1), 0);
        assert_eq!(lru.get_usage_count(2), 0);
        assert_eq!(lru.get_usage_count(3), 0);
        
        // Access way 1
        lru.access(1);
        assert_eq!(lru.get_usage_count(0), 3);
        assert_eq!(lru.get_usage_count(1), 4);
        assert_eq!(lru.get_usage_count(2), 0);
        assert_eq!(lru.get_usage_count(3), 0);
        
        // Access way 2
        lru.access(2);
        assert_eq!(lru.get_usage_count(0), 2);
        assert_eq!(lru.get_usage_count(1), 3);
        assert_eq!(lru.get_usage_count(2), 4);
        assert_eq!(lru.get_usage_count(3), 0);
        
        // Access way 3
        lru.access(3);
        assert_eq!(lru.get_usage_count(0), 1);
        assert_eq!(lru.get_usage_count(1), 2);
        assert_eq!(lru.get_usage_count(2), 3);
        assert_eq!(lru.get_usage_count(3), 4);
        
        // Access way 0 again
        lru.access(0);
        assert_eq!(lru.get_usage_count(0), 4);
        assert_eq!(lru.get_usage_count(1), 1);
        assert_eq!(lru.get_usage_count(2), 2);
        assert_eq!(lru.get_usage_count(3), 3);
        
        // Find LRU (should be way 1)
        assert_eq!(lru.find_lru(), 1);
    }
} 