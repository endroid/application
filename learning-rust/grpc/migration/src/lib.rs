pub use sea_orm_migration::prelude::*;

mod m20241209_000001_create_sudoku_puzzles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241209_000001_create_sudoku_puzzles_table::Migration),
        ]
    }
}
