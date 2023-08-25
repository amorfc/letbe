use migration::{Migrator, MigratorTrait};

use tonic::transport::Server;

use crate::{
    config::{init_environment_vars, ENV_CONFIG},
    infra::db_initializor::{DatabaseInitializer, DatabaseInitializerImpl},
    services::{
        club::club_grpc_server::ClubGrpcServer, common::grpc_server::LetGrpcServer,
        proto::LETT_FILE_DESCRIPTOR_SET, user::user_grpc_server::UserGrpcServer,
    },
};

pub mod application;
pub mod config;
pub mod infra;
pub mod services;
pub mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_environment_vars()?;
    let db = DatabaseInitializer::connect().await?;

    Migrator::up(&*db, None).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(LETT_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = format!("{}:{}", ENV_CONFIG.host, ENV_CONFIG.host_port);
    println!("Server running on  {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(UserGrpcServer::new().serve(db.clone()))
        .add_service(ClubGrpcServer::new().serve(db))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
