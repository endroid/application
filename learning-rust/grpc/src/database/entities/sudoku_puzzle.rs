/// Database entity for storing Sudoku puzzles and their solutions
/// 
/// This entity tracks unique puzzle-solution pairs for caching and analytics.
/// The puzzle_input serves as the primary key to ensure uniqueness.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sudoku_puzzles")]
pub struct Model {
    /// Primary key - original puzzle input (81 characters: dots for empty, 1-9 for filled)
    /// Example: "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"
    #[sea_orm(primary_key)]
    pub puzzle_input: String,
    
    /// Solved puzzle (81 characters: all cells filled with 1-9)
    /// Example: "534678912672195843198342567859761324426853791713924856961537428387419625245286179"
    /// NULL if puzzle was unsolvable
    pub solution: Option<String>,
    
    /// Time taken to solve the puzzle in microseconds
    /// NULL if puzzle was unsolvable or failed
    pub solve_time_us: Option<i64>,
    
    /// Estimated difficulty level: Easy, Medium, Hard, Expert
    pub difficulty: String,
    
    /// Number of empty cells in the original puzzle
    pub empty_cells: i32,
    
    /// Whether the puzzle was successfully solved
    pub is_solved: bool,
    
    /// Error message if solving failed
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: Option<String>,
    
    /// When this puzzle was first requested
    pub created_at: ChronoDateTimeUtc,
    
    /// When this puzzle was last accessed (for cache management)
    pub last_accessed_at: ChronoDateTimeUtc,
    
    /// How many times this puzzle has been requested
    #[sea_orm(default_value = 1)]
    pub access_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Check if this puzzle represents a successful solve
    pub fn is_successful(&self) -> bool {
        self.is_solved && self.solution.is_some()
    }
    
    /// Get the solve time as a Duration if available
    pub fn solve_duration(&self) -> Option<std::time::Duration> {
        self.solve_time_us.map(|us| std::time::Duration::from_micros(us as u64))
    }
}
