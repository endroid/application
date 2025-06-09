//! gRPC Learning Project Library
//! 
//! This library provides gRPC services for educational purposes, including:
//! - Sudoku solving service (with real solver implementation)
//! - Factorial calculation service
//! 
//! The library is structured with clear separation of concerns:
//! - `proto` module contains generated protocol buffer bindings
//! - `services` module contains business logic implementations
//! - `sudoku` module contains the actual Sudoku solving algorithms

pub mod proto;
pub mod services;
pub mod sudoku;

// Re-export service implementations for easier access
pub use services::{SudokuSolverService, FactorialCalculatorService};
// Re-export sudoku functionality
pub use sudoku::{SudokuBoard, SudokuSolver, SudokuComplexity, TestPuzzles};
