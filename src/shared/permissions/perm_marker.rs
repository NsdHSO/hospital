use crate::http_response::error_handler::CustomError;
use crate::http_response::{Claims, HttpCodeW};
use crate::shared::{PermissionCode, Require};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use std::marker::PhantomData;

pub trait PermMarker {
    fn code() -> &'static str;
}

impl<M: PermMarker> FromRequest for Require<M> {
    type Error = CustomError;
    type Future = Ready<Result<Self, CustomError>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        // Reuse your Claims extractor
        match Claims::from_request(req, pl).into_inner() {
            Ok(c) => {
                if c.0.perms.iter().any(|p| p == M::code()) {
                    ready(Ok(Require {
                        claims: c,
                        _m: PhantomData,
                    }))
                } else {
                    ready(Err(CustomError::new(
                        HttpCodeW::Forbidden,
                        "Missing Permissions".into(),
                    )))
                }
            }
            Err(e) => ready(Err(e)),
        }
    }
}
