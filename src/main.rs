use sea_orm::DatabaseConnection;
use tonic::transport::Server;

use crate::{
    config::{init_environment_vars, ENV_CONFIG},
    infra::db_initializor::{DatabaseInitializer, DatabaseInitializerImpl},
    services::{common::grpc_server::LetGrpcServer, user::user_grpc_server::UserGrpcServer},
};

pub mod config;
pub mod entities;
pub mod infra;
pub mod services;

pub type LetDbConnection = DatabaseConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_environment_vars()?;

    let db_conn = DatabaseInitializer::connect().await?;
    let user_gserver = UserGrpcServer::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(user_gserver.get_descriptor())
        .build()
        .unwrap();

    let addr = format!("{}:{}", ENV_CONFIG.host, ENV_CONFIG.host_port);
    println!("Server running on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(user_gserver.serve(db_conn.to_owned()))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
