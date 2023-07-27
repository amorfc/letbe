use tonic::{Request, Response, Status};

use crate::services::{
    common::request::request_validator::RequestValidator,
    proto::user::{
        user_server::User, RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData,
    },
    user::user_request::RequestUser,
};

pub struct UserService {}

#[tonic::async_trait]
impl User for UserService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        dbg!(&request);

        let user = request.into_inner();
        let request_user = RequestUser::from(user);

        RequestValidator::new(&request_user).validate_for_response()?;

        // let user = User::new(user.name, user.email, user.password);
        // let user = RegisteredUserResponseData::from(user);
        // let response = RegisterUserResponse::new(user);
        // Ok(Response::new(response))

        Ok(Response::new(RegisterUserResponse {
            data: Some(RegisteredUserResponseData {
                token: "token".to_string(),
            }),
        }))
    }
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }
}
