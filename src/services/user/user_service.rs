use tonic::{Request, Response, Status};

use crate::{
    application::managers::user::user_manager::UserManagerTrait,
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData},
        user::user_request::RequestUser,
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
        dbg!(&request);

        let user = request.into_inner();
        let request_user = RequestUser::from(user);

        RequestValidator::new(&request_user).validate_for_response()?;

        //TODO: Implement the user registration logic
        let _ = self.user_manager.user_registration().await;

        Ok(Response::new(RegisterUserResponse {
            data: Some(RegisteredUserResponseData {
                token: "token".to_string(),
            }),
        }))
    }
}

impl<T: UserManagerTrait> UserService<T> {
    pub fn new(_db_connection: LetDbConnection) -> Self {
        let user_manager = T::new();
        Self { user_manager }
    }
}
