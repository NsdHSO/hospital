use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr; // Import SeaORM's database error type
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

// Implement From for SeaORM's DbErr
impl From<DbErr> for CustomError {
    fn from(error: DbErr) -> CustomError {
        match error {
            DbErr::Conn(e) => CustomError::new(500, format!("Database connection error: {}", e)),
            DbErr::Exec(e) => CustomError::new(500, format!("Database execution error: {}", e)),
            DbErr::Query(e) => CustomError::new(500, format!("Database query error: {}", e)),
            DbErr::Json(e) => CustomError::new(500, format!("JSON error: {}", e)),
            DbErr::ConvertFromU64(e) => CustomError::new(500, format!("Conversion error: {}", e)),
            DbErr::RecordNotFound(_) => CustomError::new(404, "Record not found".to_string()),
            DbErr::Custom(e) => CustomError::new(500, format!("Custom database error: {}", e)),
            _ => CustomError::new(500, format!("Unknown database error: {:?}", error)), // Catch any other DbErr variants
        }
    }
}


impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = StatusCode::from_u16(self.error_status_code)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => json!({
                "message": "Internal server error",
                "error": self.error_message.clone()
            }).to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}