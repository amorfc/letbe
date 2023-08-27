use crate::{
    application::managers::club::club_manager::ClubManagerImpl,
    infra::db_initializor::LetDbConnection,
    services::{common::grpc_server::LetGrpcServer, proto::club::club_server::ClubServer},
};

use super::club_service::ClubService;

type TClubGrpcServer = ClubServer<ClubService<ClubManagerImpl>>;
pub struct ClubGrpcServer {}

impl ClubGrpcServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ClubGrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LetGrpcServer<TClubGrpcServer> for ClubGrpcServer {
    fn serve(&self, db_connection: LetDbConnection) -> TClubGrpcServer {
        ClubServer::new(ClubService::new(db_connection))
    }
}
