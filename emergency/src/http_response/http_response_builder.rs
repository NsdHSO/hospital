use crate::http_response::create_response::create_response;
use crate::http_response::http_code_w::HttpCodeW;
use crate::http_response::response_object::ResponseObject;

pub fn ok<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::OK)
}

pub fn created<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Created)
}

pub fn no_content<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NoContent)
}

pub fn bad_request<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::BadRequest)
}

pub fn unauthorized<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Unauthorized)
}

pub fn conflict<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Conflict)
}

pub fn not_found<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NotFound)
}

pub fn internal_server_error<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::InternalServerError)
}

pub fn not_implemented<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NotImplemented)
}
