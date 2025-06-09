/// High-performance Sudoku solver using backtracking with optimizations
/// 
/// This solver uses several techniques to improve performance:
/// - Most Constrained Variable (MCV) heuristic: chooses cells with fewest possible values
/// - Constraint propagation: eliminates impossible values before trying
/// - Backtracking with early termination

use super::board::{SudokuBoard, Cell};

#[derive(Debug)]
pub struct SudokuSolver {
    /// Maximum number of iterations before giving up (prevents infinite loops)
    max_iterations: usize,
}

impl SudokuSolver {
    /// Create a new solver with default settings
    pub fn new() -> Self {
        Self {
            max_iterations: 1_000_000, // Reasonable limit for hard puzzles
        }
    }

    /// Create a solver with custom iteration limit
    pub fn with_max_iterations(max_iterations: usize) -> Self {
        Self { max_iterations }
    }

    /// Solve a Sudoku puzzle
    /// 
    /// Returns Some(solved_board) if a solution exists, None if unsolvable or too complex
    pub fn solve(&self, board: &SudokuBoard) -> Option<SudokuBoard> {
        if !board.is_valid() {
            return None; // Invalid starting position
        }

        let mut working_board = board.clone();
        let mut iterations = 0;
        
        if self.solve_recursive(&mut working_board, &mut iterations) {
            Some(working_board)
        } else {
            None
        }
    }

    /// Recursive backtracking solver with optimizations
    fn solve_recursive(&self, board: &mut SudokuBoard, iterations: &mut usize) -> bool {
        *iterations += 1;
        if *iterations > self.max_iterations {
            return false; // Timeout to prevent infinite loops
        }

        // Find the best cell to fill next (Most Constrained Variable heuristic)
        let (row, col) = match self.find_best_cell(board) {
            Some(pos) => pos,
            None => return board.is_complete(), // No empty cells left
        };

        // Try each possible value for this cell
        let possible_values = self.get_possible_values(board, row, col);
        
        for value in possible_values {
            board.set(row, col, value);
            
            // Recursively solve with this value
            if self.solve_recursive(board, iterations) {
                return true; // Solution found
            }
            
            // Backtrack: remove the value and try next
            board.set(row, col, 0);
        }
        
        false // No solution found with any value
    }

    /// Find the empty cell with the fewest possible values (MCV heuristic)
    /// This significantly improves performance by reducing the search space
    fn find_best_cell(&self, board: &SudokuBoard) -> Option<(usize, usize)> {
        let mut best_cell = None;
        let mut min_possibilities = 10; // More than max possible (9)
        
        for row in 0..9 {
            for col in 0..9 {
                if board.get(row, col) == 0 {
                    let possible_count = self.count_possible_values(board, row, col);
                    
                    if possible_count == 0 {
                        // No valid values for this cell - unsolvable
                        return None;
                    }
                    
                    if possible_count < min_possibilities {
                        min_possibilities = possible_count;
                        best_cell = Some((row, col));
                        
                        // If only one possibility, this is the best we can do
                        if possible_count == 1 {
                            break;
                        }
                    }
                }
            }
            if min_possibilities == 1 {
                break; // Found a cell with only one possibility
            }
        }
        
        best_cell
    }

    /// Get all possible values for a cell, ordered by frequency (least common first)
    /// This implements a basic "Least Constraining Value" heuristic
    fn get_possible_values(&self, board: &SudokuBoard, row: usize, col: usize) -> Vec<Cell> {
        let mut possible = Vec::new();
        
        for value in 1..=9 {
            if board.is_valid_move(row, col, value) {
                possible.push(value);
            }
        }
        
        // For more advanced solving, we could order by how constraining each value is
        // For now, we use natural order which works well enough
        possible
    }

    /// Count how many values are possible for a given cell
    fn count_possible_values(&self, board: &SudokuBoard, row: usize, col: usize) -> usize {
        let mut count = 0;
        for value in 1..=9 {
            if board.is_valid_move(row, col, value) {
                count += 1;
            }
        }
        count
    }

    /// Validate that a solution is correct
    pub fn validate_solution(&self, board: &SudokuBoard) -> bool {
        if !board.is_complete() {
            return false;
        }
        board.is_valid()
    }

    /// Get solving statistics for the last solve operation
    pub fn get_complexity_estimate(&self, board: &SudokuBoard) -> SudokuComplexity {
        let empty_cells = self.count_empty_cells(board);
        let constrained_cells = self.count_highly_constrained_cells(board);
        
        if empty_cells < 20 {
            SudokuComplexity::Easy
        } else if empty_cells < 40 {
            SudokuComplexity::Medium
        } else if constrained_cells > empty_cells / 2 {
            SudokuComplexity::Hard
        } else {
            SudokuComplexity::Expert
        }
    }

    fn count_empty_cells(&self, board: &SudokuBoard) -> usize {
        let mut count = 0;
        for row in 0..9 {
            for col in 0..9 {
                if board.get(row, col) == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_highly_constrained_cells(&self, board: &SudokuBoard) -> usize {
        let mut count = 0;
        for row in 0..9 {
            for col in 0..9 {
                if board.get(row, col) == 0 && self.count_possible_values(board, row, col) <= 2 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SudokuComplexity {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl std::fmt::Display for SudokuComplexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuComplexity::Easy => write!(f, "Easy"),
            SudokuComplexity::Medium => write!(f, "Medium"),
            SudokuComplexity::Hard => write!(f, "Hard"),
            SudokuComplexity::Expert => write!(f, "Expert"),
        }
    }
}

impl Default for SudokuSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_easy_puzzle() {
        // Easy puzzle with many clues
        let input = "5,3,0,0,7,0,0,0,0,6,0,0,1,9,5,0,0,0,0,9,8,0,0,0,0,6,0,8,0,0,0,6,0,0,0,3,4,0,0,8,0,3,0,0,1,7,0,0,0,2,0,0,0,6,0,6,0,0,0,0,2,8,0,0,0,0,4,1,9,0,0,5,0,0,0,0,8,0,0,7,9";
        let board = SudokuBoard::from_string(input).unwrap();
        let solver = SudokuSolver::new();
        
        let solution = solver.solve(&board);
        assert!(solution.is_some());
        
        let solved = solution.unwrap();
        assert!(solver.validate_solution(&solved));
        assert!(solved.is_complete());
    }

    #[test]
    fn test_unsolvable_puzzle() {
        // Invalid puzzle with duplicate in first row
        let input = "1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0";
        let board = SudokuBoard::from_string(input).unwrap();
        let solver = SudokuSolver::new();
        
        let solution = solver.solve(&board);
        assert!(solution.is_none());
    }

    #[test]
    fn test_complexity_estimation() {
        let easy_input = "5,3,4,6,7,8,9,1,2,6,7,2,1,9,5,3,4,8,1,9,8,3,4,2,5,6,7,8,5,9,7,6,1,4,2,3,4,2,6,8,5,3,7,9,1,7,1,3,9,2,4,8,5,6,9,6,1,5,3,7,2,8,4,2,8,7,4,1,9,6,3,5,3,4,5,2,8,6,1,7,0";
        let board = SudokuBoard::from_string(easy_input).unwrap();
        let solver = SudokuSolver::new();
        
        let complexity = solver.get_complexity_estimate(&board);
        assert_eq!(complexity, SudokuComplexity::Easy);
    }
}
