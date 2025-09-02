//! gRPC server implementation

use anyhow::Result;
use tracing::info;
use std::net::SocketAddr;
use tonic::transport::Server;


pub mod services;
pub mod user_services;
pub mod conversions;

use services::UserServiceImpl;
use user_services::user_service_server::UserServiceServer;

pub async fn start_grpc_server(
    addr: SocketAddr,
    app_state: crate::AppState,
) -> Result<()> {
    info!("Starting gRPC server on {}", addr);
    let user_service = UserServiceImpl::new(app_state.clone());
    let server = Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr);

    info!("gRPC server listening on {}", addr);
    server.await?;

    Ok(())
}
