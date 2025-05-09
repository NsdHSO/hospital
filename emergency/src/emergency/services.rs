use crate::db::config::connection;
use crate::error_handler::CustomError;
use sea_orm::DatabaseConnection;

pub struct EmergencyService {
    conn: DatabaseConnection,
}

impl EmergencyService {
    pub async fn new() -> Result<Self, CustomError> {
        let conn = connection().await?; // Changed connection handling
        Ok(EmergencyService { conn: conn.clone() })
    }
}
