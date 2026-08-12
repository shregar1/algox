use crate::abstraction::AlgorithmTrait;
use super::abstraction::MatrixAlgorithmTrait;

/// Gaussian Elimination solver for system of linear equations Ax = b.
pub struct GaussianElimination;

impl GaussianElimination {
    /// Solves Ax = b using partial pivoting in O(n^3).
    /// Returns vector x if a unique solution exists.
    pub fn solve(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
        let n = a.len();
        if n == 0 || b.len() != n {
            return None;
        }

        // Build augmented matrix [A | b]
        let mut aug = vec![vec![0.0; n + 1]; n];
        for i in 0..n {
            if a[i].len() != n {
                return None;
            }
            for j in 0..n {
                aug[i][j] = a[i][j];
            }
            aug[i][n] = b[i];
        }

        // Forward elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if aug[k][i].abs() > aug[max_row][i].abs() {
                    max_row = k;
                }
            }

            if aug[max_row][i].abs() < 1e-12 {
                return None; // Singular matrix or infinite solutions
            }

            aug.swap(i, max_row);

            for k in (i + 1)..n {
                let factor = aug[k][i] / aug[i][i];
                for j in i..=n {
                    aug[k][j] -= factor * aug[i][j];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Some(x)
    }
}

impl AlgorithmTrait for GaussianElimination {
    fn name(&self) -> &'static str {
        "gaussian_elimination"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl MatrixAlgorithmTrait for GaussianElimination {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_elimination() {
        let a = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0],
        ];
        let b = vec![8.0, -11.0, -3.0];

        let sol = GaussianElimination::solve(&a, &b).unwrap();
        assert!((sol[0] - 2.0).abs() < 1e-6);
        assert!((sol[1] - 3.0).abs() < 1e-6);
        assert!((sol[2] - (-1.0)).abs() < 1e-6);
    }
}
