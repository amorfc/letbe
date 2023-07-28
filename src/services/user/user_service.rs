use entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, TransactionTrait};
use tonic::{Request, Response, Status};

use crate::services::{
    common::request::request_validator::RequestValidator,
    proto::user::{RegisterUserRequest, RegisterUserResponse, RegisteredUserResponseData},
    user::user_request::RequestUser,
};

pub struct UserService {
    db_connection: DatabaseConnection,
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

        let db = &self.db_connection;

        let txn = db.begin().await.unwrap();
        let create_user_model = user::ActiveModel {
            email: ActiveValue::set(request_user.email),
            password: ActiveValue::set(request_user.password),
            user_type: ActiveValue::Set(user::UserType::Corporation),
            name: ActiveValue::set(request_user.name),
            surname: ActiveValue::set(request_user.surname),
            ..Default::default()
        };

        create_user_model.save(&txn).await.unwrap();
        txn.commit().await.unwrap();

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
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}
