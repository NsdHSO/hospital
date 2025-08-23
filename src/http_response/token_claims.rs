use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::PermissionCode;
use actix_web::{dev::Payload, http::header, web, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

/// Represents the claims contained within a JSON Web Token (JWT).
///
/// This struct is used to deserialize the token's payload. The fields are
/// derived from the standard JWT claims, with custom fields for permissions
/// and roles.
#[derive(Debug, Deserialize)]
struct TokenClaims {
    /// The subject of the token, typically a user's unique identifier.
    sub: String,
    /// A unique identifier for the token itself.
    token_uuid: String,
    /// A list of permissions granted to the subject.
    ///
    /// The `#[serde(default)]` attribute ensures this field will be an empty
    /// vector if it is missing from the JWT payload, preventing deserialization
    /// errors.
    #[serde(default)]
    perms: Vec<String>,
    /// A list of roles assigned to the subject.
    roles: Vec<String>,
    /// The token's expiration timestamp (in seconds since the epoch).
    exp: i64,
    /// The token's "issued at" timestamp.
    iat: i64,
    /// The token's "not before" timestamp, indicating when the token becomes valid.
    nbf: i64,
}

/// A wrapper struct for `TokenClaims` that implements `actix-web`'s `FromRequest` trait.
///
/// This struct is an **extractor** that automatically handles JWT authentication
/// by decoding and validating a bearer token from the request's `Authorization` header.
pub struct Claims(TokenClaims);

impl FromRequest for Claims {
    /// The type of error returned if authentication fails.
    type Error = CustomError;
    /// The future representing the asynchronous authentication process.
    type Future = Ready<Result<Self, CustomError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 1) Bearer token
        let auth = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        let token = match auth.strip_prefix("Bearer ") {
            Some(t) if !t.is_empty() => t,
            _ => {
                return ready(Err(CustomError::new(
                    HttpCodeW::Unauthorized,
                    "Missing bearer token".to_string(),
                )));
            }
        };

        let key = match req.app_data::<web::Data<DecodingKey>>() {
            Some(k) => k.clone(),
            None => {
                return ready(Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Server Error".to_string(),
                )));
            }
        };

        let validation = Validation::new(Algorithm::RS256);

        let res = decode::<TokenClaims>(token, &key, &validation)
            .map(|d| Claims(d.claims))
            .map_err(|_| CustomError::new(HttpCodeW::Unauthorized, "Invalid token".to_string()));

        ready(res)
    }
}

/// Checks if a user has a specific permission.
///
/// This function is an authorization helper to verify that the decoded
/// claims contain a single required permission.
///
/// # Arguments
/// * `claims` - A reference to the validated `Claims` object.
/// * `perm` - The `PermissionCode` to check for.
///
/// # Returns
/// `Ok(())` if the permission is present, otherwise a `CustomError` with a Forbidden status.
///
/// # Example
/// ```
/// # use actix_web::{get, HttpResponse};
/// # use your_crate_name::{Claims, PermissionCode, require_permission};
/// #[get("/admin")]
/// async fn admin_route(claims: Claims) -> Result<HttpResponse, actix_web::Error> {
///     require_permission(&claims, PermissionCode::Admin)?;
///     Ok(HttpResponse::Ok().body("Welcome, Administrator!"))
/// }
/// ```
pub fn require_permission(claims: &Claims, perm: PermissionCode) -> Result<(), CustomError> {
    if claims.0.perms.iter().any(|p| *p == perm.as_str()) {
        Ok(())
    } else {
        Err(CustomError::new(
            HttpCodeW::Forbidden,
            "Missing Permissions".to_string(),
        ))
    }
}

/// Checks if a user has at least one of the required permissions.
///
/// This is useful for routes that can be accessed by multiple roles or with
/// different permissions.
///
/// # Arguments
/// * `claims` - A reference to the validated `Claims` object.
/// * `perms` - A slice of `PermissionCode`s to check for.
///
/// # Returns
/// `Ok(())` if any of the permissions are present, otherwise a `CustomError` with a Forbidden status.
///
/// # Example
/// ```
/// # use actix_web::{get, HttpResponse};
/// # use your_crate_name::{Claims, PermissionCode, require_any};
/// #[get("/view-data")]
/// async fn view_data_route(claims: Claims) -> Result<HttpResponse, actix_web::Error> {
///     require_any(&claims, &[PermissionCode::Read, PermissionCode::Admin])?;
///     Ok(HttpResponse::Ok().body("You can view this data."))
/// }
/// ```
pub fn require_any(claims: &Claims, perms: &[PermissionCode]) -> Result<(), CustomError> {
    if perms
        .iter()
        .any(|code| claims.0.perms.iter().any(|p| p == code.as_str()))
    {
        Ok(())
    } else {
        Err(CustomError::new(
            HttpCodeW::Forbidden,
            "Missing Permissions".to_string(),
        ))
    }
}

/// Checks if a user has all of the required permissions.
///
/// Use this when a route requires a combination of permissions to be accessed.
///
/// # Arguments
/// * `claims` - A reference to the validated `Claims` object.
/// * `perms` - A slice of `PermissionCode`s to check for.
///
/// # Returns
/// `Ok(())` if all permissions are present, otherwise a `CustomError` with a Forbidden status.
///
/// # Example
/// ```
/// # use actix_web::{get, HttpResponse};
/// # use your_crate_name::{Claims, PermissionCode, require_all};
/// #[get("/update-profile")]
/// async fn update_profile_route(claims: Claims) -> Result<HttpResponse, actix_web::Error> {
///     require_all(&claims, &[PermissionCode::Read, PermissionCode::Write])?;
///     Ok(HttpResponse::Ok().body("Profile updated successfully."))
/// }
/// ```
pub fn require_all(claims: &Claims, perms: &[PermissionCode]) -> Result<(), CustomError> {
    if perms
        .iter()
        .all(|code| claims.0.perms.iter().any(|p| p == code.as_str()))
    {
        Ok(())
    } else {
        Err(CustomError::new(
            HttpCodeW::Forbidden,
            "Missing Permissions".to_string(),
        ))
    }
}