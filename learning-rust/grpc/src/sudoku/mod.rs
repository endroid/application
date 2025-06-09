//! Sudoku solving library
//! 
//! This module provides a complete Sudoku solver implementation with:
//! - Board representation and validation
//! - High-performance backtracking solver with optimizations
//! - Difficulty estimation
//! - Test puzzles for various difficulty levels

pub mod board;
pub mod solver;
pub mod test_puzzles;

pub use board::SudokuBoard;
pub use solver::{SudokuSolver, SudokuComplexity};
pub use test_puzzles::TestPuzzles;

/// Solve a Sudoku puzzle from string format
/// 
/// Convenience function that combines parsing and solving in one call.
/// Returns the solved puzzle as a single-line string without commas.
/// Uses dots (.) for empty cells and numbers 1-9 for filled cells.
pub fn solve_puzzle_string(input: &str) -> Result<String, String> {
    let board = SudokuBoard::from_string(input)?;
    let solver = SudokuSolver::new();
    
    match solver.solve(&board) {
        Some(solved_board) => Ok(solved_board.to_string()),
        None => Err("Puzzle is unsolvable or too complex".to_string()),
    }
}

/// Validate that a string represents a valid Sudoku puzzle
pub fn validate_puzzle_string(input: &str) -> Result<(), String> {
    let board = SudokuBoard::from_string(input)?;
    if board.is_valid() {
        Ok(())
    } else {
        Err("Puzzle contains invalid constraints".to_string())
    }
}
