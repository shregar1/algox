use crate::abstraction::AlgorithmTrait;
use super::abstraction::BacktrackingAlgorithmTrait;

/// N-Queens solver using backtracking.
pub struct NQueens;

impl NQueens {
    /// Solve the N-Queens problem for `n` x `n` chessboard.
    /// Returns vector of board configurations (each configuration is a list of queen column indices per row).
    pub fn solve(n: usize) -> Vec<Vec<usize>> {
        if n == 0 {
            return Vec::new();
        }
        let mut results = Vec::new();
        let mut board = vec![0; n];
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n];
        let mut diag2 = vec![false; 2 * n];

        Self::backtrack(0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut results);
        results
    }

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut Vec<usize>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        results: &mut Vec<Vec<usize>>,
    ) {
        if row == n {
            results.push(board.clone());
            return;
        }

        for col in 0..n {
            let d1 = row + col;
            let d2 = row + n - col - 1 + n; // offset
            let d2_idx = if row >= col { row - col } else { n + col - row };

            if !cols[col] && !diag1[d1] && !diag2[d2_idx] {
                board[row] = col;
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2_idx] = true;

                Self::backtrack(row + 1, n, board, cols, diag1, diag2, results);

                cols[col] = false;
                diag1[d1] = false;
                diag2[d2_idx] = false;
            }
        }
    }
}

impl AlgorithmTrait for NQueens {
    fn name(&self) -> &'static str {
        "n_queens"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl BacktrackingAlgorithmTrait for NQueens {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_4() {
        let solutions = NQueens::solve(4);
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_n_queens_8() {
        let solutions = NQueens::solve(8);
        assert_eq!(solutions.len(), 92);
    }
}
