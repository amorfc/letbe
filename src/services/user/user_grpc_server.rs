use crate::{
    application::managers::user::user_manager::UserManagerImpl,
    infra::db_initializor::LetDbConnection,
    services::{common::grpc_server::LetGrpcServer, proto::user::user_server::UserServer},
};

use super::user_service::UserService;

type TUserGrpcServer = UserServer<UserService<UserManagerImpl>>;
pub struct UserGrpcServer {}

impl UserGrpcServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UserGrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LetGrpcServer<TUserGrpcServer> for UserGrpcServer {
    fn serve(&self, db_connection: LetDbConnection) -> TUserGrpcServer {
        UserServer::new(UserService::new(db_connection))
    }
}
