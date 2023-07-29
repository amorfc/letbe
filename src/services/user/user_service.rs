use entity::user;
use sea_orm::ActiveValue;
use tonic::{Request, Response, Status};

use crate::{
    application::repositories::user::user_repository::UserRepository,
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData},
        user::user_request::RequestUser,
    },
};

pub struct UserService {
    user_repository: UserRepository,
}

#[tonic::async_trait]
impl crate::services::proto::user::user_server::User for UserService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        dbg!(&request);

        let user = request.into_inner();
        let request_user = RequestUser::from(user);

        RequestValidator::new(&request_user).validate_for_response()?;

        let create_user_model = user::ActiveModel {
            email: ActiveValue::set(request_user.email),
            password: ActiveValue::set(request_user.password),
            user_type: ActiveValue::Set(user::UserType::Corporation),
            name: ActiveValue::set(request_user.name),
            surname: ActiveValue::set(request_user.surname),
            ..Default::default()
        };

        let user_create_result = self.user_repository.create_user(create_user_model).await;

        Ok(Response::new(RegisterUserResponse {
            data: Some(RegisteredUserResponseData {
                token: "token".to_string(),
            }),
        }))
    }
}

impl UserService {
    pub fn new(db_connection: LetDbConnection) -> Self {
        let user_repository = UserRepository::new(db_connection.clone());
        Self { user_repository }
    }
}
