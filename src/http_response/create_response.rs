use crate::http_response::http_code_w::HttpCodeW;
use crate::http_response::response_object::ResponseObject;

/// Creates a ResponseObject with the provided message and HTTP status code.
///
/// This is the core helper function that all other response functions use internally.
/// It constructs a ResponseObject struct containing the message and HTTP status code,
/// which can be serialized to JSON with both fields.
///
/// # Arguments 
///
/// * `message`: The response payload of type T (must implement Serialize for JSON serialization)
/// * `code`: The HTTP status code as HttpCodeW enum variant
///
/// returns: ResponseObject<T> - A response object with the provided message and status code
///
/// # Examples 
///
/// ```rust
/// let response = create_response("Custom message", HttpCodeW::OK);
/// let custom_response = create_response(CustomData { value: 42 }, HttpCodeW::Accepted);
/// // Serializes to: {"message": "Custom message", "code": 200}
/// ```
pub fn create_response<T>(message: T, code: HttpCodeW) -> ResponseObject<T> {
    ResponseObject { message, code }
}