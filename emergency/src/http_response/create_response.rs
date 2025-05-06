use crate::http_response::http_code_w::HttpCodeW;
use crate::http_response::response_object::ResponseObject;

pub fn create_response<T>(message: T, code: HttpCodeW) -> ResponseObject<T> {
    ResponseObject { message, code }
}
