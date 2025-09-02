//! REST API server implementation using Axum

use anyhow::Result;
use axum::{
    routing::{get},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::info;

pub mod handlers;
pub mod middleware;
pub mod routes;


pub fn create_router(app_state: crate::AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .nest("/api/v1", routes::v1::create_v1_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(middleware::auth::AuthLayer::new())
        )
        .with_state(app_state)
}
pub async fn start_rest_server(
    addr: SocketAddr,
    app_state: crate::AppState,
) -> Result<()> {
    info!("Starting REST API server on {}", addr);
    let app = create_router(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
