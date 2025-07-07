#[cfg(test)]
mod db_tests {
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
    use crate::tests::db_config::setup_test_db;

    #[tokio::test]
    async fn test_database_connection() {
        let db: DatabaseConnection = setup_test_db().await;
        
        // Try a simple query to verify the connection is working
        let result = db.execute(Statement::from_string(
            DbBackend::Postgres,
            "SELECT 1".to_string(),
        ))
        .await;
        
        // If the query succeeds, the connection is working
        assert!(result.is_ok(), "Should be able to run a simple query");
        
        // Just check that we got some result back
        let query_result = result.unwrap();
        assert!(query_result.rows_affected() >= 0, "Query should execute successfully");
    }
}
