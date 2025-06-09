/// Database integration module for the gRPC Sudoku service
/// 
/// This module provides a clean separation between the domain logic (Sudoku solving)
/// and persistence concerns. It handles:
/// - Database connection management
/// - Puzzle persistence and retrieval
/// - Migration management
/// - Connection pooling

pub mod entities;
pub mod persistence;

pub use entities::*;
pub use persistence::PersistenceService;

use migration;

use sea_orm::{Database, DatabaseConnection, DbErr};

/// Database configuration and connection management
pub struct DatabaseConfig {
    pub database_url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://root:root@localhost:5432/nestjs".to_string(),
        }
    }
}

impl DatabaseConfig {
    /// Create a new database configuration
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }
    
    /// Create a connection to the database
    pub async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        println!("🗄️  Connecting to database: {}", self.database_url);
        let db = Database::connect(&self.database_url).await?;
        println!("✅ Database connection established");
        Ok(db)
    }
}

/// Initialize the database with migrations
pub async fn initialize_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm_migration::MigratorTrait;
    
    println!("🔄 Running database migrations...");
    
    // Run all pending migrations
    migration::Migrator::up(db, None).await?;
    
    println!("✅ Database migrations completed");
    Ok(())
}
