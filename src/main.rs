use migration::{Migrator, MigratorTrait};

use tonic::transport::Server;

use crate::{
    config::{init_environment_vars, ENV_CONFIG},
    infra::db_initializor::{DatabaseInitializer, DatabaseInitializerImpl},
    services::{common::grpc_server::LetGrpcServer, user::user_grpc_server::UserGrpcServer},
};

pub mod application;
pub mod config;
pub mod infra;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_environment_vars()?;
    let db = DatabaseInitializer::connect().await?;

    Migrator::up(&db, None).await?;

    let user_gserver = UserGrpcServer::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(user_gserver.get_descriptor())
        .build()
        .unwrap();

    let addr = format!("{}:{}", ENV_CONFIG.host, ENV_CONFIG.host_port);
    println!("Server running on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(user_gserver.serve(db))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
