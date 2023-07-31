use tonic::{Request, Response, Status};

use crate::{
    application::managers::user::user_manager::UserManagerTrait,
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData},
        user::user_request::NewUser,
    },
};

pub struct UserService<T: UserManagerTrait> {
    user_manager: T,
}

#[tonic::async_trait]
impl<T: UserManagerTrait + Send + Sync + 'static> crate::services::proto::user::user_server::User
    for UserService<T>
{
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        let user: RegisterUserRequest = request.into_inner();
        let input_create_user = NewUser::from(user);

        RequestValidator::new(&input_create_user).validate_for_response()?;

        let registered_user: RegisteredUserResponseData = self
            .user_manager
            .user_registration(input_create_user)
            .await
            .map_err(Status::internal)?
            .into();

        let data = Some(registered_user);
        let response = RegisterUserResponse { data };
        Ok(Response::new(response))
    }
}

impl<T: UserManagerTrait> UserService<T> {
    pub fn new(db_connection: LetDbConnection) -> Self {
        let user_manager = T::new(db_connection);
        Self { user_manager }
    }
}
