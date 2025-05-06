use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum HttpCodeW {
    // Successful Responses
    OK = 200,
    Created = 201,
    NoContent = 204,

    // Client Errors
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    UnprocessableEntity = 422,

    // Server Errors
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
}
