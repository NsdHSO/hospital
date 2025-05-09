use sea_orm::{Database, DatabaseConnection};
use once_cell::sync::OnceCell;
use std::env;
use crate::error_handler::CustomError;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() -> Result<(), CustomError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| CustomError::new(500, format!("Failed to connect to DB: {}", e)))?;

    DB.set(conn)
        .map_err(|_| CustomError::new(500, "DB already initialized".to_string()))?;

    Ok(())
}

pub async fn connection() -> Result<&'static DatabaseConnection, CustomError> {
    DB.get().ok_or_else(|| CustomError::new(500, "Database not initialized".to_string()))
}
