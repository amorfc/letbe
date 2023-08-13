
use tonic::{Request, Response, Status};

use crate::{
    application::managers::user::user_manager::{UserManagerImpl, UserManagerTrait},
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{
            user_server::User as UserServer, RegisterUserRequest, RegisterUserResponse,
        },
        user::user_request::NewUser,
    },
};

pub struct UserService<T: UserManagerTrait> {
    manager: T,
}

impl UserService<UserManagerImpl> {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            manager: UserManagerImpl::new(db_connection),
        }
    }
}

#[tonic::async_trait]
impl UserServer for UserService<UserManagerImpl> {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        let user: RegisterUserRequest = request.into_inner();
        let input_create_user = NewUser::from(user);

        RequestValidator::new(&input_create_user).validate_for_response()?;

        let registered_user = self
            .manager
            .user_registration(input_create_user.into())
            .await
            .map_err(Status::internal)?;

        let response_data = registered_user.into();

        let data = Some(response_data);
        let response = RegisterUserResponse { data };
        Ok(Response::new(response))
    }
}
