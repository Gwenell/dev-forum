pub mod migrations;

use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::env;
use dotenv::dotenv;

pub type DbPool = DatabaseConnection;

pub async fn establish_connection() -> Result<DbPool, DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;
    
    // Run migrations
    migrations::Migrator::up(&db, None).await?;
    
    Ok(db)
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    migrations::Migrator::up(db, None).await?;
    Ok(())
} 