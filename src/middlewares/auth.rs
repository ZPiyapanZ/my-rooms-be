use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::jwt::Claims;

pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        JwtMiddleware { secret }
    }
}

// Implement the middleware transformation
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareMiddleware<S>;
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let secret = self.secret.clone();
        Box::pin(async move { Ok(JwtMiddlewareMiddleware { service, secret }) })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        let auth_header = req.headers().get("Authorization").cloned();
        let mut is_auth = false;

        // Validate token
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                // Expecting "Bearer <token>"
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Strip "Bearer " prefix

                    let validation = Validation::default();
                    let decoding_key = DecodingKey::from_secret(secret.as_ref());

                    // Validate the token
                    if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                        req.request().extensions_mut().insert(token_data.claims.sub);
                        is_auth = true;
                    }
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            if is_auth {
                return fut.await;
            }

            Err(ErrorUnauthorized("Invalid or missing JWT"))
        })
    }
}
