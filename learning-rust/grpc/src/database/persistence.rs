/// Persistence service for Sudoku puzzles
/// 
/// This service provides a clean abstraction over database operations for Sudoku puzzles.
/// It handles caching, deduplication, and analytics while keeping the domain logic separate.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, PaginatorTrait,
};

use chrono::Utc;
use std::time::Duration;

use crate::sudoku::SudokuComplexity;
use super::entities::{SudokuPuzzle, SudokuPuzzleModel, SudokuPuzzleActiveModel};

/// Result of a puzzle lookup operation
#[derive(Debug, Clone)]
pub struct PuzzleLookupResult {
    pub found: bool,
    pub puzzle: Option<SudokuPuzzleModel>,
}

/// Input data for storing a new puzzle result
#[derive(Debug, Clone)]
pub struct PuzzleStoreInput {
    pub puzzle_input: String,
    pub solution: Option<String>,
    pub solve_time: Option<Duration>,
    pub difficulty: SudokuComplexity,
    pub empty_cells: usize,
    pub is_solved: bool,
    pub error_message: Option<String>,
}

/// Service for managing Sudoku puzzle persistence
#[derive(Debug)]
pub struct PersistenceService {
    db: DatabaseConnection,
}

impl PersistenceService {
    /// Create a new persistence service with a database connection
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Look up a puzzle by its input
    /// 
    /// This allows us to check if we've seen this exact puzzle before
    /// and can return a cached solution instead of re-solving.
    pub async fn find_puzzle_by_input(&self, puzzle_input: &str) -> Result<PuzzleLookupResult, DbErr> {
        let puzzle = SudokuPuzzle::find_by_id(puzzle_input.to_string())
            .one(&self.db)
            .await?;

        if let Some(mut puzzle) = puzzle {
            // Update access tracking
            self.update_access_tracking(&puzzle.puzzle_input).await?;
            puzzle.last_accessed_at = Utc::now();
            puzzle.access_count += 1;
            
            Ok(PuzzleLookupResult {
                found: true,
                puzzle: Some(puzzle),
            })
        } else {
            Ok(PuzzleLookupResult {
                found: false,
                puzzle: None,
            })
        }
    }

    /// Store a new puzzle and its solution (or failure) in the database
    pub async fn store_puzzle_result(&self, input: PuzzleStoreInput) -> Result<SudokuPuzzleModel, DbErr> {
        let now = Utc::now();

        let active_model = SudokuPuzzleActiveModel {
            puzzle_input: Set(input.puzzle_input.clone()),
            solution: Set(input.solution),
            solve_time_us: Set(input.solve_time.map(|d| d.as_micros() as i64)),
            difficulty: Set(input.difficulty.to_string()),
            empty_cells: Set(input.empty_cells as i32),
            is_solved: Set(input.is_solved),
            error_message: Set(input.error_message),
            created_at: Set(now),
            last_accessed_at: Set(now),
            access_count: Set(1),
            ..Default::default()
        };

        let result = active_model.insert(&self.db).await?;
        
        println!("💾 Stored new puzzle in database:");
        println!("   Input: {}...", &result.puzzle_input[..20]);
        println!("   Difficulty: {}", result.difficulty);
        println!("   Solved: {}", result.is_solved);
        
        Ok(result)
    }

    /// Update access tracking for an existing puzzle
    async fn update_access_tracking(&self, puzzle_input: &str) -> Result<(), DbErr> {
        use sea_orm::{ActiveModelTrait, IntoActiveModel};
        
        if let Some(puzzle) = SudokuPuzzle::find_by_id(puzzle_input.to_string()).one(&self.db).await? {
            let access_count = puzzle.access_count;
            let mut active_model = puzzle.into_active_model();
            active_model.last_accessed_at = Set(Utc::now());
            active_model.access_count = Set(access_count + 1);
            active_model.update(&self.db).await?;
        }
        
        Ok(())
    }

    /// Get statistics about stored puzzles
    pub async fn get_puzzle_statistics(&self) -> Result<PuzzleStatistics, DbErr> {
        let total_puzzles = SudokuPuzzle::find().count(&self.db).await?;
        let solved_puzzles = SudokuPuzzle::find()
            .filter(crate::database::entities::sudoku_puzzle::Column::IsSolved.eq(true))
            .count(&self.db)
            .await?;
        
        // Get average solve time for solved puzzles
        let avg_solve_time = if solved_puzzles > 0 {
            // This would need a more complex query in practice
            Some(Duration::from_millis(100)) // Placeholder
        } else {
            None
        };

        Ok(PuzzleStatistics {
            total_puzzles,
            solved_puzzles,
            failed_puzzles: total_puzzles - solved_puzzles,
            avg_solve_time,
        })
    }
}

/// Statistics about puzzles stored in the database
#[derive(Debug, Clone)]
pub struct PuzzleStatistics {
    pub total_puzzles: u64,
    pub solved_puzzles: u64,
    pub failed_puzzles: u64,
    pub avg_solve_time: Option<Duration>,
}

impl std::fmt::Display for PuzzleStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Puzzles: {} total, {} solved, {} failed",
            self.total_puzzles, self.solved_puzzles, self.failed_puzzles
        )?;
        
        if let Some(avg_time) = self.avg_solve_time {
            write!(f, ", avg time: {:?}", avg_time)?;
        }
        
        Ok(())
    }
}
