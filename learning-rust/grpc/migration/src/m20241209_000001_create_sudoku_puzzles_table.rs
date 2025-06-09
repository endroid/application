use sea_orm_migration::prelude::*;

/// Migration to create the sudoku_puzzles table
/// 
/// This table stores unique puzzle-solution pairs with metadata for:
/// - Caching solved puzzles to avoid re-computation
/// - Analytics on puzzle difficulty and solve times
/// - Request tracking and access patterns
/// - Uses puzzle_input as primary key for natural uniqueness

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
            Table::create()
            .table(SudokuPuzzles::Table)
            .if_not_exists()
            .col(
            ColumnDef::new(SudokuPuzzles::PuzzleInput)
            .string_len(81)
            .not_null()
            .primary_key(),
            )
                    .col(
                        ColumnDef::new(SudokuPuzzles::Solution)
                            .string_len(81)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::SolveTimeUs)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::Difficulty)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::EmptyCells)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::IsSolved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::LastAccessedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SudokuPuzzles::AccessCount)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for common queries
        manager
            .create_index(
                Index::create()
                    .name("idx_sudoku_puzzles_difficulty")
                    .table(SudokuPuzzles::Table)
                    .col(SudokuPuzzles::Difficulty)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sudoku_puzzles_created_at")
                    .table(SudokuPuzzles::Table)
                    .col(SudokuPuzzles::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SudokuPuzzles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SudokuPuzzles {
    Table,
    PuzzleInput,
    Solution,
    SolveTimeUs,
    Difficulty,
    EmptyCells,
    IsSolved,
    ErrorMessage,
    CreatedAt,
    LastAccessedAt,
    AccessCount,
}
