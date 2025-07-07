use sea_orm::{Database, DatabaseConnection};
use std::env;

// Database connection for tests - using either Docker or existing infrastructure
pub async fn get_test_db() -> Result<DatabaseConnection, String> {
    // Use TEST_DATABASE_URL if set (which it will be in Docker)
    let db_url = match env::var("TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            // Fallback to infrastructure selection when running locally without Docker
            let test_db_env = env::var("TEST_DB_ENV").unwrap_or_else(|_| "local".to_string());
            
            match test_db_env.as_str() {
                "local" => "postgres://test_user:test_password@localhost:5450/hospital_test".to_string(),
                "fat" => "postgres://fat_user:fat_pass@localhost:5440/fat_db".to_string(),
                "uat" => "postgres://uat_user:uat_pass@localhost:5441/uat_db".to_string(),
                "preprod" => "postgres://preprod_user:preprod_pass@localhost:5442/preprod_db".to_string(),
                "prod" => "postgres://prod_user:prod_pass@localhost:5443/prod_db".to_string(),
                "railway" => "postgres://prod_user:prod_pass@localhost:5444/prod_db".to_string(),
                // Use your local database URL for direct IDE test runs
                "ide" => env::var("DATABASE_URL").unwrap_or_else(|_| {
                    "postgres://postgres:postgres@localhost:5432/postgres".to_string()
                }),
                _ => "postgres://test_user:test_password@localhost:5450/hospital_test".to_string(),
            }
        }
    };
    
    // Print connection info (redacting password)
    let safe_url = db_url.replace(db_url.split(':').nth(2).unwrap_or("").split('@').next().unwrap_or(""), "****");
    println!("Connecting to test database at: {}", safe_url);
    
    // Connect to the database
    match Database::connect(&db_url).await {
        Ok(conn) => {
            println!("Successfully connected to database");
            Ok(conn)
        },
        Err(e) => {
            let error_msg = format!("Failed to connect to test database: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

// Helper function to setup a clean test database state
pub async fn setup_test_db() -> DatabaseConnection {
    // Try to connect to the test database
    match get_test_db().await {
        Ok(db) => {
            // Run migrations if needed
            // This would depend on how you handle migrations
            // migration::Migrator::up(&db, None).await.expect("Failed to run migrations");
            
            db
        },
        Err(e) => {
            // If we can't connect to the test database, try connecting to the development database
            println!("Couldn't connect to test database, trying development database: {}", e);
            
            // Set TEST_DB_ENV to "ide" to use development database
            // This is unsafe because it modifies global process state and could cause
            // undefined behavior in multi-threaded contexts
            unsafe {
                std::env::set_var("TEST_DB_ENV", "ide");
            }
            
            // Try again with the development database
            match get_test_db().await {
                Ok(db) => db,
                Err(e) => {
                    // If we still can't connect, panic with a helpful message
                    panic!("Cannot connect to any database for testing. Make sure you have a database available or Docker running. Error: {}", e);
                }
            }
        }
    }
}

// Helper function to clean up test database after tests
pub async fn cleanup_after_test(db: &DatabaseConnection) {
    // Delete all test data after test runs
    // This is a simple approach, you might want to implement more sophisticated cleanup
    // based on your specific test needs
    
    println!("Cleaning up test data...");
    
// Example: sea_orm::EntityTrait::delete_many() calls for your test entities
    // await those calls to ensure data is cleaned up
}
