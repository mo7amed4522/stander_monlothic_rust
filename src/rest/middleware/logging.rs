//! Request logging middleware

use axum::{
    extract::Request,
    response::Response,
};
use tower::{Layer, Service};
use std::task::{Context, Poll};
use tracing::{info};
use std::time::Instant;


#[derive(Clone)]
pub struct RequestLoggingLayer;

impl RequestLoggingLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestLoggingLayer {
    type Service = RequestLoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestLoggingMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for RequestLoggingMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let method = request.method().clone();
        let uri = request.uri().clone();
        let start_time = Instant::now();

        let future = self.inner.call(request);

        Box::pin(async move {
            let response = future.await?;
            let duration = start_time.elapsed();

            info!(
                method = %method,
                uri = %uri,
                status = %response.status(),
                duration_ms = %duration.as_millis(),
                "HTTP request completed"
            );

            Ok(response)
        })
    }
}
