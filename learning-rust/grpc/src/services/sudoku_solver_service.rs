use tonic::{Request, Response, Status};
use crate::proto::sudoku::sudoku_server::Sudoku;
use crate::proto::sudoku::Board;
use crate::sudoku::{SudokuBoard, SudokuSolver};
use crate::database::{PersistenceService, persistence::{PuzzleStoreInput}};

/// Service that handles Sudoku puzzle solving requests
/// 
/// This service uses a high-performance backtracking solver to solve Sudoku puzzles.
/// It includes database persistence for caching solutions and analytics.
#[derive(Debug)]
pub struct SudokuSolverService {
    solver: SudokuSolver,
    persistence: Option<PersistenceService>,
}

impl SudokuSolverService {
    /// Create a new Sudoku solver service without database persistence
    pub fn new() -> Self {
        Self {
            solver: SudokuSolver::new(),
            persistence: None,
        }
    }

    /// Create a service with database persistence
    pub fn with_persistence(persistence: PersistenceService) -> Self {
        Self {
            solver: SudokuSolver::new(),
            persistence: Some(persistence),
        }
    }

    /// Create a service with custom solver settings and optional persistence
    pub fn with_solver_and_persistence(solver: SudokuSolver, persistence: Option<PersistenceService>) -> Self {
        Self { solver, persistence }
    }
}

impl Default for SudokuSolverService {
    fn default() -> Self {
        Self::new()
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
        
        // Check database cache first if persistence is enabled
        if let Some(ref persistence) = self.persistence {
            match persistence.find_puzzle_by_input(puzzle_input).await {
                Ok(lookup_result) if lookup_result.found => {
                    if let Some(cached_puzzle) = lookup_result.puzzle {
                        if cached_puzzle.is_solved {
                            println!("💾 Found cached solution in database");
                            if let Some(solve_time) = cached_puzzle.solve_duration() {
                                println!("   ⏱️  Original solve time: {:?}", solve_time);
                            }
                            println!("   🎯 Cached solution: {}", cached_puzzle.solution.as_ref().unwrap());
                            
                            return Ok(Response::new(Board {
                                values: cached_puzzle.solution.unwrap(),
                            }));
                        } else {
                            println!("💾 Found cached failure - puzzle is unsolvable");
                            return Err(Status::failed_precondition(
                                cached_puzzle.error_message.unwrap_or_else(|| 
                                    "Puzzle is unsolvable (from cache)".to_string()
                                )
                            ));
                        }
                    }
                }
                Ok(_) => {
                    println!("💾 No cached solution found, will solve and store");
                }
                Err(e) => {
                    println!("⚠️  Database lookup failed: {}, continuing without cache", e);
                }
            }
        }
        
        // Parse the input board from string format
        let board = match SudokuBoard::from_string(puzzle_input) {
            Ok(board) => board,
            Err(error_msg) => {
                println!("❌ Invalid board format: {}", error_msg);
                
                // Store failure in database if persistence enabled
                if let Some(ref persistence) = self.persistence {
                    let store_input = PuzzleStoreInput {
                        puzzle_input: puzzle_input.clone(),
                        solution: None,
                        solve_time: None,
                        difficulty: crate::sudoku::SudokuComplexity::Easy, // Default for invalid input
                        empty_cells: 0,
                        is_solved: false,
                        error_message: Some(error_msg.clone()),
                    };
                    let _ = persistence.store_puzzle_result(store_input).await;
                }
                
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
                let solution = board.to_string();
                
                // Store completed puzzle in database if persistence enabled
                if let Some(ref persistence) = self.persistence {
                    let store_input = PuzzleStoreInput {
                        puzzle_input: puzzle_input.clone(),
                        solution: Some(solution.clone()),
                        solve_time: Some(std::time::Duration::from_micros(0)), // Already solved
                        difficulty: complexity,
                        empty_cells,
                        is_solved: true,
                        error_message: None,
                    };
                    let _ = persistence.store_puzzle_result(store_input).await;
                }
                
                return Ok(Response::new(Board { values: solution }));
            } else {
                println!("❌ Puzzle complete but invalid");
                
                // Store invalid puzzle in database if persistence enabled
                if let Some(ref persistence) = self.persistence {
                    let store_input = PuzzleStoreInput {
                        puzzle_input: puzzle_input.clone(),
                        solution: None,
                        solve_time: None,
                        difficulty: complexity,
                        empty_cells,
                        is_solved: false,
                        error_message: Some("Puzzle is complete but contains invalid constraints".to_string()),
                    };
                    let _ = persistence.store_puzzle_result(store_input).await;
                }
                
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
                    
                    // Store successful solution in database if persistence enabled
                    if let Some(ref persistence) = self.persistence {
                        let store_input = PuzzleStoreInput {
                            puzzle_input: puzzle_input.clone(),
                            solution: Some(solution.clone()),
                            solve_time: Some(solve_time),
                            difficulty: complexity,
                            empty_cells,
                            is_solved: true,
                            error_message: None,
                        };
                        if let Err(e) = persistence.store_puzzle_result(store_input).await {
                            println!("⚠️  Failed to store solution in database: {}", e);
                        }
                    }
                    
                    Ok(Response::new(Board {
                        values: solution,
                    }))
                } else {
                    let solve_time = start_time.elapsed();
                    println!("❌ Solver produced invalid solution (time: {:?})", solve_time);
                    
                    // Store failure in database if persistence enabled
                    if let Some(ref persistence) = self.persistence {
                        let store_input = PuzzleStoreInput {
                            puzzle_input: puzzle_input.clone(),
                            solution: None,
                            solve_time: Some(solve_time),
                            difficulty: complexity,
                            empty_cells,
                            is_solved: false,
                            error_message: Some("Solver produced invalid solution".to_string()),
                        };
                        let _ = persistence.store_puzzle_result(store_input).await;
                    }
                    
                    Err(Status::internal("Solver produced invalid solution"))
                }
            }
            None => {
                let solve_time = start_time.elapsed();
                println!("⚠️  Puzzle unsolvable or too complex (time: {:?})", solve_time);
                
                // Store failure in database if persistence enabled
                if let Some(ref persistence) = self.persistence {
                    let store_input = PuzzleStoreInput {
                        puzzle_input: puzzle_input.clone(),
                        solution: None,
                        solve_time: Some(solve_time),
                        difficulty: complexity,
                        empty_cells,
                        is_solved: false,
                        error_message: Some("Puzzle is unsolvable or too complex".to_string()),
                    };
                    if let Err(e) = persistence.store_puzzle_result(store_input).await {
                        println!("⚠️  Failed to store failure in database: {}", e);
                    }
                }
                
                Err(Status::failed_precondition(
                    "Puzzle is unsolvable or too complex to solve within time limits"
                ))
            }
        }
    }
}
