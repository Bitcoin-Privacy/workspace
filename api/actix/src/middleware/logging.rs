use actix_web::{
    dev::{Service, ServiceResponse},
    dev::{ServiceRequest, Transform},
    Error,
};
use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

// Define your middleware struct. It can contain configurations or shared data.
pub struct LoggingMiddleware;

// Implement Transform trait for your middleware
impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService { service }))
    }
}

// Define the service that is transformed by your middleware
pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = <S as Service<ServiceRequest>>::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Log the request line and headers
        println!("Request Method: {}", req.method());
        println!("Request URI: {}", req.uri());
        println!("Headers:");
        for (key, value) in req.headers() {
            println!("  {}: {:?}", key, value);
        }

        self.service.call(req)
    }
}
