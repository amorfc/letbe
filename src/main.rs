use tonic::transport::Server;

use crate::{
    env_config::{set_environment, ENV_CONFIG},
    services::{common::grpc_server::LettGrpcServer, user::user_grpc_server::UserGrpcServer},
};

pub mod entities;
pub mod env_config;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_environment()?;

    let user_gserver = UserGrpcServer::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(user_gserver.get_descriptor())
        .build()
        .unwrap();

    let addr = format!("{}:{}", ENV_CONFIG.host, ENV_CONFIG.host_port);
    println!("Server running on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(user_gserver.serve())
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
