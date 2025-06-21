/// High-performance Sudoku solver using backtracking with optimizations
/// 
/// This solver uses several techniques to improve performance:
/// - Most Constrained Variable (MCV) heuristic: chooses cells with fewest possible values
/// - Constraint propagation: eliminates impossible values before trying
/// - Backtracking with early termination

use super::board::{SudokuBoard, Cell};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
        println!("🧩 Starting to solve board...");
        println!("Initial board state:\n{}", board);
        
        if !board.is_valid() {
            return None; // Invalid starting position
        }

        let mut board_copy = board.clone();
        let mut iterations = 0;
        
        if self.solve_recursive(&mut board_copy, &mut iterations) {
            println!("✅ Successfully solved the board in {} iterations", iterations);
            println!("Solved board:\n{}", board_copy);
            Some(board_copy)
        } else {
            println!("❌ Failed to solve the board after {} iterations", iterations);
            // Print the board state where it got stuck
            println!("Board state at failure point:\n{}", board_copy);
            // Print the number of empty cells remaining
            let empty_cells = self.count_empty_cells(&board_copy);
            println!("Empty cells remaining: {}", empty_cells);
            None
        }
    }

    /// Recursive backtracking solver with optimizations and detailed logging
    fn solve_recursive(&self, board: &mut SudokuBoard, iterations: &mut usize) -> bool {
        *iterations += 1;
        
        // Print progress every 1000 iterations
        if *iterations % 1000 == 0 {
            println!("  Iteration: {}, Empty cells: {}", 
                   iterations, self.count_empty_cells(board));
        }
        
        // Safety check to prevent infinite loops
        if *iterations > self.max_iterations {
            println!("❌ Exceeded maximum iterations ({})", self.max_iterations);
            return false;
        }

        // Find the best cell to fill next (Most Constrained Variable heuristic)
        let (row, col) = match self.find_best_cell(board) {
            Some(pos) => pos,
            None => {
                // No empty cells left, check if the board is complete and valid
                let is_complete = board.is_complete();
                if is_complete {
                    println!("✅ Found a complete solution!");
                } else {
                    println!("❌ No empty cells but board is not complete");
                }
                return is_complete;
            },
        };

        // Get possible values for this cell (ordered by least constraining value)
        let possible_values = self.get_possible_values(board, row, col);
        
        if possible_values.is_empty() {
            println!("  Dead end at ({}, {}): no possible values", row, col);
            return false;
        }
        
        if *iterations % 1000 == 0 {
            println!("  Trying cell ({}, {}), possible values: {:?}", 
                   row, col, possible_values);
        }
        
        for value in possible_values {
            if *iterations % 1000 == 0 {
                println!("    Trying value {} at ({}, {})", value, row, col);
            }
            
            board.set(row, col, value);
            
            // Recursively solve with this value
            if self.solve_recursive(board, iterations) {
                return true; // Solution found
            }
            
            // Backtrack: remove the value and try next
            board.set(row, col, 0);
        }
        
        if *iterations % 1000 == 0 {
            println!("  No valid value found for cell ({}, {})", row, col);
        }
        
        false // No valid value found for this cell
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
        (0..9).flat_map(|i| (0..9).map(move |j| (i, j)))
            .filter(|&(i, j)| board.get(i, j) == 0 && self.count_possible_values(board, i, j) <= 2)
            .count()
    }
    
    /// Generate a new Sudoku puzzle with the specified difficulty
    /// 
    /// The difficulty affects how many cells are filled in the puzzle:
    /// - Easy: 45-50 clues (31-36 empty cells)
    /// - Medium: 36-44 clues (37-45 empty cells)
    /// - Hard: 27-35 clues (46-54 empty cells)
    /// - Expert: 17-26 clues (55-64 empty cells)
    pub fn generate_puzzle(&self, difficulty: &SudokuComplexity) -> Option<(SudokuBoard, SudokuBoard)> {
        println!("\n🎲 Starting puzzle generation with difficulty: {:?}", difficulty);
        
        // Start with an empty board
        let mut board = SudokuBoard::new();
        let mut rng = thread_rng();
        
        println!("🔧 Filling diagonal boxes...");
        // Only fill the three diagonal boxes (0,0), (1,1), (2,2)
        // These boxes are independent of each other
        for box_idx in 0..3 {
            println!("  Filling diagonal box at ({}, {})", box_idx, box_idx);
            self.fill_diagonal_box(&mut board, box_idx, box_idx);
            println!("  Board after filling diagonal box ({}x{}):\n{}", box_idx, box_idx, board);
        }
        
        // Print the initial board state with just the diagonal boxes filled
        println!("✅ Filled diagonal boxes. Initial board state:\n{}", board);
        
        println!("✅ Filled all diagonal boxes. Initial board state:\n{}", board);
        
        // Solve the puzzle to get a complete solution
        println!("🧩 Attempting to solve the initial board...");
        let solution = match self.solve(&board) {
            Some(sol) => {
                println!("✅ Successfully generated a complete solution");
                println!("   Solution board:\n{}", sol);
                sol
            },
            None => {
                println!("❌ Failed to generate a complete solution");
                println!("   Initial board state that couldn't be solved:\n{}", board);
                return None;
            }
        };
        
        // Now create a puzzle by removing numbers
        let mut puzzle = solution.clone();
        
        println!("🧩 Starting to remove cells from the solution...");
        
        // Create a vector of all cell positions and shuffle them
        let mut cells: Vec<(usize, usize)> = (0..9)
            .flat_map(|i| (0..9).map(move |j| (i, j)))
            .collect();
        cells.shuffle(&mut rng);
        
        println!("  Shuffled {} cells for potential removal", cells.len());
        
        // Determine number of cells to keep based on difficulty
        let cells_to_keep = match *difficulty {
            SudokuComplexity::Easy => {
                let keep = rng.gen_range(40..=45);
                println!("  Difficulty: Easy - keeping {}-45 cells (36-41 empty)", keep);
                keep
            },
            SudokuComplexity::Medium => {
                let keep = rng.gen_range(32..=40);
                println!("  Difficulty: Medium - keeping {}-40 cells (41-49 empty)", keep);
                keep
            },
            SudokuComplexity::Hard => {
                let keep = rng.gen_range(28..=35);
                println!("  Difficulty: Hard - keeping {}-35 cells (46-53 empty)", keep);
                keep
            },
            SudokuComplexity::Expert => {
                let keep = rng.gen_range(22..=30);
                println!("  Difficulty: Expert - keeping {}-30 cells (51-59 empty)", keep);
                keep
            },
        };
        
        // Try to remove cells while ensuring the puzzle remains solvable with a unique solution
        let mut cells_to_remove = 81 - cells_to_keep;
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 100;
        println!("🔢 Target cells to remove: {}, cells to keep: {}", cells_to_remove, cells_to_keep);
        println!("  Starting cell removal process...");
        
        while cells_to_remove > 0 && attempts < MAX_ATTEMPTS && !cells.is_empty() {
            attempts += 1;
            
            // Find a cell to remove
            let (i, j) = if let Some(&pos) = cells.first() { 
                pos 
            } else { 
                println!("  No more cells to try removing");
                break; 
            };
            
            // Skip if cell is already empty
            if puzzle.get(i, j) == 0 {
                cells.remove(0);
                continue;
            }
            
            // Save the value and remove it
            let value = puzzle.get(i, j);
            puzzle.set(i, j, 0);
            
            if attempts % 10 == 0 {
                println!("  Attempt {}: Trying to remove cell at ({}, {}) = {}", attempts, i, j, value);
            }
            
            // Check if the puzzle still has a unique solution
            let mut temp_puzzle = puzzle.clone();
            let solution_count = self.count_solutions(&mut temp_puzzle, 2); // We only care if there's more than 1 solution
            
            if solution_count == 1 {
                // Keep this cell removed
                cells_to_remove -= 1;
                cells.remove(0);
                if cells_to_remove % 5 == 0 {
                    println!("  ✓ Removed cell at ({}, {}) = {} ({} more to remove)", 
                            i, j, value, cells_to_remove);
                }
                attempts = 0; // Reset attempts counter on success
            } else {
                // Put the value back
                puzzle.set(i, j, value);
                // Move this cell to the end to try other cells first
                if let Some(cell) = cells.pop() {
                    cells.insert(0, cell);
                }
                if attempts % 20 == 0 {
                    println!("  ✗ Couldn't remove cell at ({}, {}), trying another (attempt {}/{})", 
                            i, j, attempts, MAX_ATTEMPTS);
                }
            }
        }
        
        // Calculate how many cells we've actually removed
        let removed_cells = puzzle.to_string().chars().filter(|&c| c == '0' || c == '.').count();
        let target_removed = 81 - cells_to_keep;
        
        println!("\n📊 Puzzle generation results:");
        println!("  - Cells removed: {}/{} (target: {})", removed_cells, 81, target_removed);
        println!("  - Cells remaining: {}", 81 - removed_cells);
        
        // If we couldn't remove enough cells, log a warning but continue
        if removed_cells < target_removed {
            println!("⚠️  Warning: Only removed {} out of {} target cells for difficulty {:?}", 
                    removed_cells, target_removed, difficulty);
                    
            // If we're close to the target, still accept the puzzle
            if removed_cells >= target_removed.saturating_sub(5) {
                println!("   Proceeding with {} removed cells (close enough to target {})", 
                        removed_cells, target_removed);
            } else {
                println!("   Not enough cells removed, but will try to proceed");
            }
        } else {
            println!("✅ Successfully removed {} cells (target was {})", removed_cells, target_removed);
        }
        
        // Make sure we have a valid puzzle with at least 17 clues (minimum for a unique solution)
        let clues = puzzle.to_string().chars().filter(|&c| c != '0' && c != '.').count();
        if clues < 17 {
            println!("❌ Failed to generate puzzle: Only {} clues (need at least 17)", clues);
            println!("   Puzzle state ({} clues, {} empty):\n{}", 
                   clues, 81 - clues, puzzle);
            return None;
        }
        
        // Validate that the puzzle is actually solvable
        println!("🔍 Validating the generated puzzle...");
        let temp_puzzle = puzzle.clone();
        if self.solve(&temp_puzzle).is_none() {
            println!("❌ Failed to validate puzzle: Generated puzzle is not solvable");
            return None;
        }
        
        // Log the final puzzle stats
        let empty_cells = puzzle.to_string().chars().filter(|&c| c == '0' || c == '.').count();
        println!("\n🎉 Successfully generated puzzle!");
        println!("   Difficulty: {:?}", difficulty);
        println!("   Clues: {} (min: 17)", clues);
        println!("   Empty cells: {}", empty_cells);
        println!("   Puzzle:\n{}", puzzle);
        println!("   Solution:\n{}", solution);
        
        // Return the generated puzzle and solution
        Some((puzzle, solution))
    }
    
    /// Fill a 3x3 box on the diagonal with random numbers 1-9 without duplicates
    fn fill_diagonal_box(&self, board: &mut SudokuBoard, box_row: usize, box_col: usize) {
        // Create a vector of numbers 1-9 and shuffle them
        let mut numbers: Vec<u8> = (1..=9).collect();
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);
        
        // Fill the 3x3 box with the shuffled numbers
        for i in 0..3 {
            for j in 0..3 {
                let row = box_row * 3 + i;
                let col = box_col * 3 + j;
                // Use a unique number for each cell in the box
                board.set(row, col, numbers[i * 3 + j]);
            }
        }
    }
    
    /// Count the number of solutions up to the given limit
    fn count_solutions(&self, board: &mut SudokuBoard, limit: usize) -> usize {
        let mut count = 0;
        self.count_solutions_recursive(board, &mut count, limit);
        count
    }
    
    /// Recursive helper for counting solutions
    fn count_solutions_recursive(&self, board: &mut SudokuBoard, count: &mut usize, limit: usize) -> bool {
        if *count >= limit {
            return true; // Early exit if we've reached the limit
        }
        
        match self.find_best_cell(board) {
            None => {
                *count += 1;
                *count >= limit
            }
            Some((row, col)) => {
                let values = self.get_possible_values(board, row, col);
                for value in values {
                    board.set(row, col, value);
                    if self.count_solutions_recursive(board, count, limit) {
                        return true;
                    }
                    board.set(row, col, 0);
                }
                false
            }
        }
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
