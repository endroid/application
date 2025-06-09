// Service implementations for the gRPC server
pub mod sudoku_solver_service;
pub mod factorial_calculator_service;

// Re-export service structs for easy access
pub use sudoku_solver_service::SudokuSolverService;
pub use factorial_calculator_service::FactorialCalculatorService;
