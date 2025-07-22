use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Europe;
use nanoid::nanoid;
use sea_orm::DbErr;

pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS: f64 = 6371.0;
    let (lat1_rad, lon1_rad) = (lat1.to_radians(), lon1.to_radians());
    let (lat2_rad, lon2_rad) = (lat2.to_radians(), lon2.to_radians());
    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    let a =
        (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS * c
}
pub fn generate_ic() -> i32 {
    generate_ic_with_length(Some(30)) // Default to 6 digits
}

pub fn generate_ic_with_length(length: Option<usize>) -> i32 {
    let length = length.unwrap_or(9);
    // Generate numeric string using nanoid
    let numeric_string = nanoid!(length, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);

    // Ensure the string isn't too long for i32 (max 10 digits for i32)
    let safe_length = std::cmp::min(length, 9);
    let trimmed_string = if numeric_string.len() > safe_length {
        &numeric_string[0..safe_length]
    } else {
        &numeric_string
    };

    // Parse the string into an i32, with a fallback in case of errors
    trimmed_string.parse::<i32>().unwrap_or_else(|_| {
        // If parsing fails (very unlikely), generate a simpler fallback
        let fallback = nanoid!(4, &['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        fallback.parse::<i32>().unwrap_or(1000) // Final fallback value
    })
}

/// Checks if a database error is due to a duplicate key constraint violation.
///
/// This function examines the database operation result and determines if it failed
/// due to a unique constraint violation. If so, it increments the attempts counter
/// to facilitate retry logic. For other errors, it converts them to CustomError.
///
/// # Type Parameters
///
/// * `T`: The type of the successful result (e.g., database model, entity)
///
/// # Arguments
///
/// * `attempts`: Mutable reference to the retry counter that gets incremented on duplicate key errors
/// * `result`: The database operation result to examine (Result<T, DbErr>)
///
/// returns: Option<Result<T, CustomError>> - Some(Ok(value)) on success, Some(Err(error)) on non-duplicate errors, None on duplicate key (for retry)
///
/// # Examples
///
/// ```rust
/// let mut retry_count = 0;
/// let db_result: Result<User, DbErr> = user_repository.create(new_user).await;
///
/// if let Some(final_result) = check_if_is_duplicate_key(&mut retry_count, db_result) {
///     return final_result; // Either success or non-retryable error
/// }
/// // If None returned, it was a duplicate key - continue with retry logic
///
/// // Works with any type
/// let product_result: Result<Product, DbErr> = product_repository.create(new_product).await;
/// if let Some(final_result) = check_if_is_duplicate_key(&mut retry_count, product_result) {
///     return final_result;
/// }
/// ```
pub fn check_if_is_duplicate_key_from_data_base<T>(
    attempts: &mut usize,
    result: Result<T, DbErr>,
) -> Option<Result<T, CustomError>> {
    match result {
        Ok(value) => Some(Ok(value)),
        Err(e) => {
            // Check if the error is a unique constraint violation
            // The exact string to check for might vary slightly depending on the database
            println!("Error occured while checking for duplicate key: {e}");
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                // It's a unique constraint violation, increment attempts for retry logic
                *attempts += 1;
                // Return None to indicate this is a retryable duplicate key error
                None
            } else {
                // Some other execution error, return it
                Some(Err(CustomError::from(e)))
            }
        }
    }
}

pub fn now_time() -> NaiveDateTime {
    chrono::Utc::now()
        .with_timezone(&Europe::Bucharest)
        .naive_local()
}

/// Parses a date string from AppointmentRequestBody into a DateTime<Utc> object.
///
/// This function attempts to parse the string using common date formats:
/// - ISO 8601: "2025-07-22T14:30:00"
/// - Standard format: "2025-07-22 14:30:00"
/// - Alternative formats if needed
///
/// # Arguments
///
/// * `date_str`: The date string to parse
///
/// # Returns
///
/// * `Result<DateTime<Utc>, CustomError>`: The parsed DateTime or a formatting error
///
/// # Examples
///
/// ```rust
/// let appointment_data = AppointmentRequestBody {
///     appointment_date: "2025-07-22 14:30:00".to_string(),
///     // other fields...
/// };
///
/// let date = parse_appointment_date(&appointment_data.appointment_date)?;
/// ```
pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, CustomError> {
    // Try parsing as standard format "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
    }

    // Try parsing as ISO 8601 format
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try alternative format "DD/MM/YYYY HH:MM:SS"
    if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, "%d/%m/%Y %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
    }

    // Try date-only format and assume midnight time
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        if let Some(datetime) = date.and_hms_opt(0, 0, 0) {
            return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
        }
    }

    // If all parsing attempts fail, return an error
    Err(CustomError::new(
        HttpCodeW::BadRequest,
        format!("Unable to parse date: {date_str}"),
    ))
}
