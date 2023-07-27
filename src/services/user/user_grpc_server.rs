use crate::services::{
    common::{
        base::{GrpcServerConfig, GrpcServerConfigNewParam},
        grpc_server::LetGrpcServer,
    },
    proto::user::{user_server::UserServer, USER_FILE_DESCRIPTOR_SET},
};

use super::user_service::UserService;

type TUserGrpcServer = UserServer<UserService>;
pub struct UserGrpcServer {
    pub config: GrpcServerConfig,
}

impl UserGrpcServer {
    pub fn get_descriptor(&self) -> &'static [u8] {
        self.config.file_descriptor_set
    }
}

impl Default for UserGrpcServer {
    fn default() -> Self {
        let config_param = GrpcServerConfigNewParam {
            file_descriptor_set: USER_FILE_DESCRIPTOR_SET,
        };

        let config = GrpcServerConfig::new(config_param);

        Self { config }
    }
}

impl LetGrpcServer<TUserGrpcServer> for UserGrpcServer {
    fn serve(&self) -> TUserGrpcServer {
        UserServer::new(UserService::new())
    }
}
