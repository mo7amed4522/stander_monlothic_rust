use anyhow::Result;
use stander_monlothic_rust::{initialize_app, AppState};
use tokio::signal;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Rust Monolithic Application...");
    let app_state = match initialize_app().await {
        Ok(state) => {
            info!("Application initialized successfully");
            state
        }
        Err(e) => {
            error!("Failed to initialize application: {}", e);
            return Err(e);
        }
    };
    let rest_server = start_rest_server(app_state.clone());
    let grpc_server = start_grpc_server(app_state.clone());
    tokio::select! {
        result = rest_server => {
            if let Err(e) = result {
                error!("REST server error: {}", e);
            }
        }
        result = grpc_server => {
            if let Err(e) = result {
                error!("gRPC server error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal, stopping servers...");
        }
    }
    info!("Application stopped");
    Ok(())
}

async fn start_rest_server(app_state: AppState) -> Result<()> {
    use stander_monlothic_rust::rest::start_rest_server;
    use std::net::SocketAddr;
    let addr: SocketAddr = format!("{}:{}",
        app_state.config.server.host,
        app_state.config.server.rest_port
    ).parse()?;
    start_rest_server(addr, app_state).await
}

async fn start_grpc_server(app_state: AppState) -> Result<()> {
    use stander_monlothic_rust::grpc::start_grpc_server;
    use std::net::SocketAddr;
    let addr: SocketAddr = format!("{}:{}",
        app_state.config.server.host,
        app_state.config.server.grpc_port
    ).parse()?;
    start_grpc_server(addr, app_state).await
}
