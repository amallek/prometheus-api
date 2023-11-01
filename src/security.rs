use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// Implement your own x-auth token verification
fn verify_token(tok: &str) -> Result<(), ()> {
    if tok != "my_password" {
        return Err(());
    }
    Ok(())
}

pub struct Default;

impl<S, B> Transform<S, ServiceRequest> for Default
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DefaultMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DefaultMiddleware { service }))
    }
}

pub struct DefaultMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for DefaultMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("x-auth");

        if token.is_none() {
            println!("Missing x-auth token");
            return Box::pin(async move {
                Err(Error::from(actix_web::error::ErrorUnauthorized(
                    "Missing x-auth token",
                )))
            });
        }

        if let Err(_) = verify_token(token.unwrap().to_str().unwrap()) {
            println!("Token verification failed");
            return Box::pin(async move {
                Err(Error::from(actix_web::error::ErrorUnauthorized(
                    "Token verification failed",
                )))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
