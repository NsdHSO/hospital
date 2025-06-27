use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
// Import SeaORM's database error type
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;
// Import this

use crate::http_response::{HttpCodeW, create_response};
// Import logging macros

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: HttpCodeW,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: HttpCodeW, error_message: String) -> CustomError {
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

// Implement std::error::Error for CustomError
impl StdError for CustomError {}

// Implement From for SeaORM's DbErr
impl From<DbErr> for CustomError {
    fn from(error: DbErr) -> CustomError {
        match error {
            DbErr::Conn(e) => {
                let msg = format!("Database connection error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            DbErr::Exec(e) => {
                let msg = format!("Database execution error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            DbErr::Query(e) => {
                let msg = format!("Database query error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            DbErr::Json(e) => {
                let msg = format!("JSON error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            DbErr::ConvertFromU64(e) => {
                let msg = format!("Conversion error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            DbErr::RecordNotFound(_) => {
                CustomError::new(HttpCodeW::NotFound, "Record not found".to_string())
            } // Not an error that needs logging at ERROR level
            DbErr::Custom(e) => {
                let msg = format!("Custom database error: {e}");
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
            _ => {
                let msg = format!("Unknown database error: {error:?}" );
                print!("{msg}"); // Log the error
                CustomError::new(HttpCodeW::InternalServerError, msg)
            }
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        // Log the error when it's being converted to an HTTP response
        print!(
            "Responding with error: Status={:?}, Message={}",
            self.error_status_code, self.error_message
        );

        // Create a ResponseObject using the error message and mapped HttpCodeW
        let response_object = create_response(self.error_message.clone(), self.error_status_code);
        println!("ResponseObject: {:?}", response_object);
        // Build the HttpResponse based on the HttpCodeW
        let status_code = StatusCode::from_u16(self.error_status_code as u16)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        HttpResponse::build(status_code).json(response_object)
    }
}
