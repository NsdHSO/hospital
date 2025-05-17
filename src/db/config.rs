use sea_orm::{Database, DatabaseConnection};
use once_cell::sync::OnceCell;
use std::env;
use std::io::Write;
use crate::error_handler::CustomError;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();


pub async fn init() -> Result<DatabaseConnection, CustomError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Attempting to connect to database...");

    // Create a task to display progress while connecting
    let progress_task = tokio::spawn(async {
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut count = 0;
        loop {
            print!("\r{} Connection in progress... ({} seconds)",
                   spinner[count % spinner.len()],
                   count / 2);
            std::io::stdout().flush().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            count += 1;
        }
    });

    // Attempt the actual connection
    let connection_result = Database::connect(&db_url).await;

    // Cancel the progress display
    progress_task.abort();
    print!("\r");
    std::io::stdout().flush().unwrap();

    // Handle the connection result
    let conn = connection_result
        .map_err(|e| CustomError::new(500, format!("Failed to connect to DB: {}", e)))?;

    println!("Successfully connected to database!");

    // Store the connection in OnceCell
    DB.set(conn.clone()) // Clone the connection to store it
        .map_err(|_| CustomError::new(500, "DB already initialized".to_string()))?;

    Ok(conn) // Return the connection
}

pub async fn connection() -> Result<&'static DatabaseConnection, CustomError> {
    DB.get().ok_or_else(|| CustomError::new(500, "Database not initialized".to_string()))
}
