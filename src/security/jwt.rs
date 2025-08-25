use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::body::EitherBody;
use actix_web::{http::header::AUTHORIZATION, Error, HttpMessage, HttpResponse};
use awc::Client;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;

#[derive(Clone)]
pub struct JwtAuth {
    pub auth_base_url: String,
}

impl JwtAuth {
    pub fn new(auth_base_url: impl Into<String>) -> Self {
        Self { auth_base_url: auth_base_url.into() }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
            auth_base_url: self.auth_base_url.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
    auth_base_url: String,
}

#[derive(Debug, Serialize)]
struct IntrospectRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct IntrospectResponse {
    active: bool,
    sub: Option<String>,
    token_uuid: Option<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let auth_base_url = self.auth_base_url.clone();

        Box::pin(async move {
            let token = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(|s| s.to_string());

            let Some(token) = token else {
                return Ok(req
                    .into_response(HttpResponse::Unauthorized().finish())
                    .map_into_right_body());
            };

            let client = Client::default();
            let url = format!("{}/v1/auth/introspect", auth_base_url.trim_end_matches('/'));
            let mut resp = client
                .post(url)
                .send_json(&IntrospectRequest { token: token.clone() })
                .await
                // Map the awc error to your custom error
                .map_err(|_| CustomError::new(HttpCodeW::Unauthorized, "Failed to connect to auth service".to_string()))?;

            if !resp.status().is_success() {
                // Return CustomError for non-success status
                return Err(Error::from(CustomError::new(HttpCodeW::Unauthorized, "Introspection API returned non-success status".to_string())));
            }

            let body: IntrospectResponse = resp
                .json()
                .await
                // Map the JSON deserialization error to your custom error
                .map_err(|_| CustomError::new(HttpCodeW::Unauthorized, "Failed to parse JSON response from introspection API".to_string()))?;

            if !body.active {
                // Return CustomError for an inactive token
                return Err(Error::from(CustomError::new(HttpCodeW::Unauthorized, "Token is not active".to_string())));
            }

            if let Some(sub) = body.sub {
                req.extensions_mut().insert(sub);
            }
            if let Some(uuid) = body.token_uuid {
                req.extensions_mut().insert(uuid);
            }

            svc.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}

