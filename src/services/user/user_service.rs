use tonic::{Request, Response, Status};

use crate::services::user::proto::user::{
    user_server::User, RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData,
};

pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        dbg!(&request);

        Ok(Response::new(RegisterUserResponse {
            data: Some(RegisteredUserResponseData {
                token: "token".to_string(),
            }),
        }))

        // Err(Status::unimplemented("Not implemented"))
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self {}
    }
}
