use crate::{
    application::managers::user::user_manager::UserManagerImpl,
    infra::db_initializor::LetDbConnection,
    services::{
        common::{
            base::{GrpcServerConfig, GrpcServerConfigNewParam},
            grpc_server::LetGrpcServer,
        },
        proto::user::{user_server::UserServer, USER_FILE_DESCRIPTOR_SET},
    },
};

use super::user_service::UserService;

type TUserGrpcServer = UserServer<UserService<UserManagerImpl>>;
pub struct UserGrpcServer {}

impl UserGrpcServer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn config() -> GrpcServerConfig {
        let config_param = GrpcServerConfigNewParam {
            file_descriptor_set: USER_FILE_DESCRIPTOR_SET,
        };

        GrpcServerConfig::new(config_param)
    }
    pub fn descriptor() -> &'static [u8] {
        Self::config().file_descriptor_set
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
