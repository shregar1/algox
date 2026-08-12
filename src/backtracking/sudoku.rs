use crate::abstraction::AlgorithmTrait;
use super::abstraction::BacktrackingAlgorithmTrait;

/// 9x9 Sudoku Solver using Backtracking with Constraint Propagation.
pub struct SudokuSolver;

impl SudokuSolver {
    /// Solves 9x9 Sudoku in-place. `0` represents an empty cell.
    /// Returns `true` if a valid solution was found.
    pub fn solve(grid: &mut [[u8; 9]; 9]) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c] == 0 {
                    for num in 1..=9 {
                        if Self::is_valid(grid, r, c, num) {
                            grid[r][c] = num;
                            if Self::solve(grid) {
                                return true;
                            }
                            grid[r][c] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn is_valid(grid: &[[u8; 9]; 9], row: usize, col: usize, val: u8) -> bool {
        for i in 0..9 {
            if grid[row][i] == val || grid[i][col] == val {
                return false;
            }
        }
        let box_r = (row / 3) * 3;
        let box_c = (col / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                if grid[box_r + r][box_c + c] == val {
                    return false;
                }
            }
        }
        true
    }
}

impl AlgorithmTrait for SudokuSolver {
    fn name(&self) -> &'static str {
        "sudoku_solver"
    }

    fn len(&self) -> usize {
        81
    }

    fn clear(&mut self) {}
}

impl BacktrackingAlgorithmTrait for SudokuSolver {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_solver() {
        let mut ss = SudokuSolver;
        assert_eq!(ss.name(), "sudoku_solver");
        assert_eq!(ss.len(), 81);
        ss.clear();

        let mut grid: [[u8; 9]; 9] = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        assert!(SudokuSolver::solve(&mut grid));
        assert_eq!(grid[0][2], 4);
    }

    #[test]
    fn test_sudoku_unsolvable() {
        // Grid with conflicting numbers in same row and empty cell that cannot be filled
        let mut invalid_grid: [[u8; 9]; 9] = [
            [1, 2, 3, 4, 5, 6, 7, 8, 0], // missing 9
            [0, 0, 0, 0, 0, 0, 0, 0, 9], // 9 in last column conflicts with filling 9 in row 0
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(!SudokuSolver::solve(&mut invalid_grid));
    }
}
