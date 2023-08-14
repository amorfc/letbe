use tonic::{Request, Response, Status};

use crate::{
    application::managers::user::user_manager::{UserManagerImpl, UserManagerTrait},
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{
            user_server::User as UserServer, LoginUserRequest, LoginUserResponse,
            RegisterUserRequest, RegisterUserResponse,
        },
        user::register::register_request::NewUser,
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
        let new_user = NewUser::from(request.into_inner());

        RequestValidator::new(&new_user).validate_for_response()?;

        let registered_user = self
            .manager
            .user_registration(new_user)
            .await
            .map_err(Status::internal)?;

        let response_data = registered_user.into();

        let data = Some(response_data);
        let response = RegisterUserResponse { data };
        Ok(Response::new(response))
    }

    async fn login_user(
        &self,
        _request: Request<LoginUserRequest>,
    ) -> Result<Response<LoginUserResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}
