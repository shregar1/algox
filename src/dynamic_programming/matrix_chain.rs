use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// Matrix Chain Multiplication — find optimal parenthesization order.
pub struct MatrixChain;

impl MatrixChain {
    /// Given dimensions `dims` where matrix i has size dims[i] × dims[i+1],
    /// returns the minimum number of scalar multiplications.
    pub fn min_multiplications(dims: &[usize]) -> usize {
        let n = dims.len() - 1; // number of matrices
        if n == 0 { return 0; }
        let mut dp = vec![vec![0usize; n]; n];
        // chain length
        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                dp[i][j] = usize::MAX;
                for k in i..j {
                    let cost = dp[i][k]
                        + dp[k + 1][j]
                        + dims[i] * dims[k + 1] * dims[j + 1];
                    if cost < dp[i][j] {
                        dp[i][j] = cost;
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}

impl AlgorithmTrait for MatrixChain {
    fn name(&self) -> &'static str {
        "matrix_chain"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for MatrixChain {
    fn description(&self) -> &'static str {
        "Matrix Chain Multiplication: find the parenthesization minimising scalar multiplications."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_chain() {
        // Matrices: 10×30, 30×5, 5×60, 60×10
        // Optimal: ((A·B)·(C·D)) costs 1500+3000+500=5000? verified by algorithm
        assert_eq!(MatrixChain::min_multiplications(&[10, 30, 5, 60, 10]), 5000);
    }

    #[test]
    fn test_matrix_chain_single() {
        assert_eq!(MatrixChain::min_multiplications(&[5, 10]), 0);
    }
}
