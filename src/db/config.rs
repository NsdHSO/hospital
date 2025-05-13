use sea_orm::{Database, DatabaseConnection};
use once_cell::sync::OnceCell;
use std::env;
use crate::error_handler::CustomError;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() -> Result<DatabaseConnection, CustomError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| CustomError::new(500, format!("Failed to connect to DB: {}", e)))?;

    // Store the connection in OnceCell
    DB.set(conn.clone()) // Clone the connection to store it
        .map_err(|_| CustomError::new(500, "DB already initialized".to_string()))?;

    Ok(conn) // Return the connection
}

pub async fn connection() -> Result<&'static DatabaseConnection, CustomError> {
    DB.get().ok_or_else(|| CustomError::new(500, "Database not initialized".to_string()))
}
