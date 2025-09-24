use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use futures_util::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct Subject {
    pub sub: String,
    pub token_uuid: String,
}

impl FromRequest for Subject {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        // The JwtAuth middleware must have populated this
        if let Some(subj) = req.extensions().get::<Subject>() {
            return ready(Ok(subj.clone()));
        }

        // Fallback: if only raw strings were inserted (older behavior)
        let sub = req.extensions().get::<String>().cloned();
        // There may be multiple String types in extensions; try to also find token_uuid by key type
        // Since we cannot distinguish, return Unauthorized if typed Subject is missing
        match sub {
            Some(_) => ready(Err(actix_web::error::ErrorUnauthorized("subject missing token uuid"))),
            None => ready(Err(actix_web::error::ErrorUnauthorized("unauthenticated"))),
        }
    }
}