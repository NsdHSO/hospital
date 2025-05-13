use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
// Import SeaORM's database error type
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use std::error::Error as StdError; // Import this

use crate::http_response::{create_response, HttpCodeW};
use log::{error, info};
// Import logging macros

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

// Implement std::error::Error for CustomError
impl StdError for CustomError {}

// Implement From for SeaORM's DbErr
impl From<DbErr> for CustomError {
    fn from(error: DbErr) -> CustomError {
        match error {
            DbErr::Conn(e) => {
                let msg = format!("Database connection error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            DbErr::Exec(e) => {
                let msg = format!("Database execution error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            DbErr::Query(e) => {
                let msg = format!("Database query error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            DbErr::Json(e) => {
                let msg = format!("JSON error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            DbErr::ConvertFromU64(e) => {
                let msg = format!("Conversion error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            DbErr::RecordNotFound(_) => CustomError::new(404, "Record not found".to_string()), // Not an error that needs logging at ERROR level
            DbErr::Custom(e) => {
                let msg = format!("Custom database error: {}", e);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
            _ => {
                let msg = format!("Unknown database error: {:?}", error);
                print!("{}", msg); // Log the error
                CustomError::new(500, msg)
            }
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        // Log the error when it's being converted to an HTTP response
        print!(
            "Responding with error: Status={}, Message={}",
            self.error_status_code, self.error_message
        );

        // Map the CustomError status code to HttpCodeW
        let http_code = match self.error_status_code {
            200 => HttpCodeW::OK,
            201 => HttpCodeW::Created,
            204 => HttpCodeW::NoContent,

            400 => HttpCodeW::BadRequest,
            401 => HttpCodeW::Unauthorized,
            403 => HttpCodeW::Forbidden,
            404 => HttpCodeW::NotFound,
            409 => HttpCodeW::Conflict,
            422 => HttpCodeW::UnprocessableEntity,

            500 => HttpCodeW::InternalServerError,
            501 => HttpCodeW::NotImplemented,
            502 => HttpCodeW::BadGateway,
            503 => HttpCodeW::ServiceUnavailable,
            504 => HttpCodeW::GatewayTimeout,

            _ => HttpCodeW::InternalServerError,
        };

        // Create a ResponseObject using the error message and mapped HttpCodeW
        let response_object = create_response(self.error_message.clone(), http_code);
        println!("ResponseObject: {:?}", response_object);
        // Build the HttpResponse based on the HttpCodeW
        let status_code = StatusCode::from_u16(http_code as u16)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        HttpResponse::build(status_code).json(response_object)
    }
}