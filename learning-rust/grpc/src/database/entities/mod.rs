/// Database entities for the gRPC Sudoku service
/// 
/// This module contains all database entity definitions using SeaORM.
/// Entities are defined code-first and migrations are generated from them.

pub mod sudoku_puzzle;

pub use sudoku_puzzle::Entity as SudokuPuzzle;
pub use sudoku_puzzle::Model as SudokuPuzzleModel;
pub use sudoku_puzzle::ActiveModel as SudokuPuzzleActiveModel;
