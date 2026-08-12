use crate::abstraction::AlgorithmTrait;
use super::abstraction::MatrixAlgorithmTrait;

/// 2D Matrix structure for fundamental linear algebra operations.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, initial: f64) -> Self {
        Self {
            rows,
            cols,
            data: vec![initial; rows * cols],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r * self.cols + c] = val;
    }

    /// Matrix multiplication in O(r1 * c1 * c2).
    pub fn multiply(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }

        let mut result = Matrix::new(self.rows, other.cols, 0.0);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(i, k);
                for j in 0..other.cols {
                    let b = other.get(k, j);
                    let current = result.get(i, j);
                    result.set(i, j, current + a * b);
                }
            }
        }
        Some(result)
    }

    /// Matrix transposition in O(r * c).
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows, 0.0);
        for r in 0..self.rows {
            for c in 0..self.cols {
                result.set(c, r, self.get(r, c));
            }
        }
        result
    }
}

impl AlgorithmTrait for Matrix {
    fn name(&self) -> &'static str {
        "matrix_ops"
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.rows = 0;
        self.cols = 0;
    }
}

impl MatrixAlgorithmTrait for Matrix {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![2.0, 0.0, 1.0, 2.0]);
        let res = a.multiply(&b).unwrap();
        assert_eq!(res.data, vec![4.0, 4.0, 10.0, 8.0]);
    }

    #[test]
    fn test_matrix_transpose() {
        let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let t = a.transpose();
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }
}
