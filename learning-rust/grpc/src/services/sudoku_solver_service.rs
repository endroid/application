use tonic::{Request, Response, Status};
use crate::proto::sudoku::sudoku_server::Sudoku;
use crate::proto::sudoku::Board;
use crate::sudoku::{SudokuBoard, SudokuSolver};

/// Service that handles Sudoku puzzle solving requests
/// 
/// This service uses a high-performance backtracking solver to solve Sudoku puzzles.
/// It accepts puzzles in comma-separated format where 0 represents empty cells.
#[derive(Debug, Default)]
pub struct SudokuSolverService {
    solver: SudokuSolver,
}

impl SudokuSolverService {
    /// Create a new Sudoku solver service
    pub fn new() -> Self {
        Self {
            solver: SudokuSolver::new(),
        }
    }

    /// Create a service with custom solver settings
    pub fn with_solver(solver: SudokuSolver) -> Self {
        Self { solver }
    }
}

#[tonic::async_trait]
impl Sudoku for SudokuSolverService {
    /// Solve a Sudoku puzzle using advanced backtracking algorithms
    /// 
    /// # Input Format
    /// Single-line string of 81 characters (dots for empty cells, 1-9 for filled)
    /// Example: "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"
    /// 
    /// # Returns
    /// - Success: Board with all cells filled (complete solution)
    /// - Error: If puzzle is invalid, unsolvable, or too complex
    async fn solve(&self, request: Request<Board>) -> Result<Response<Board>, Status> {
        let input_board = request.into_inner();
        let puzzle_input = &input_board.values;
        
        println!("🧩 Received Sudoku solve request");
        println!("   Input: {}", puzzle_input);
        
        // Parse the input board from string format
        let board = match SudokuBoard::from_string(puzzle_input) {
            Ok(board) => board,
            Err(error_msg) => {
                println!("❌ Invalid board format: {}", error_msg);
                return Err(Status::invalid_argument(format!(
                    "Invalid board format: {}", error_msg
                )));
            }
        };

        // Count empty cells for difficulty estimation
        let empty_cells = puzzle_input.chars().filter(|&c| c == '.' || c == '0').count();
        let complexity = self.solver.get_complexity_estimate(&board);
        println!("   📊 Empty cells: {}, Estimated difficulty: {}", empty_cells, complexity);

        // Check if the puzzle is already complete
        if board.is_complete() {
            if board.is_valid() {
                println!("✅ Puzzle already solved and valid");
                return Ok(Response::new(Board {
                    values: board.to_string(),
                }));
            } else {
                println!("❌ Puzzle complete but invalid");
                return Err(Status::invalid_argument(
                    "Puzzle is complete but contains invalid constraints"
                ));
            }
        }

        // Measure solving time
        let start_time = std::time::Instant::now();
        println!("🔍 Starting solve process...");

        // Attempt to solve the puzzle
        match self.solver.solve(&board) {
            Some(solved_board) => {
                let solve_time = start_time.elapsed();
                
                // Verify the solution is correct
                if self.solver.validate_solution(&solved_board) {
                    let solution = solved_board.to_string();
                    println!("✅ Puzzle solved successfully!");
                    println!("   ⏱️  Solve time: {:?}", solve_time);
                    println!("   🎯 Solution: {}", solution);
                    
                    Ok(Response::new(Board {
                        values: solution,
                    }))
                } else {
                    let solve_time = start_time.elapsed();
                    println!("❌ Solver produced invalid solution (time: {:?})", solve_time);
                    Err(Status::internal("Solver produced invalid solution"))
                }
            }
            None => {
                let solve_time = start_time.elapsed();
                println!("⚠️  Puzzle unsolvable or too complex (time: {:?})", solve_time);
                Err(Status::failed_precondition(
                    "Puzzle is unsolvable or too complex to solve within time limits"
                ))
            }
        }
    }
}
