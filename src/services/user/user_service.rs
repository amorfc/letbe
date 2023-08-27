use tonic::{Request, Response, Status};

use crate::{
    application::managers::{
        authn::authn_manager::{AuthnManagerImpl, AuthnManagerTrait, NewJwtParams},
        user::user_manager::{UserManagerImpl, UserManagerTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        proto::user::{
            user_server::User as UserServer, LoginUserRequest, LoginUserResponse,
            RegisterUserRequest, RegisterUserResponse,
        },
        user::register::request::NewUser,
    },
};

use super::login::request::LoginUser;

pub struct UserService<T: UserManagerTrait> {
    manager: T,
    authn_manager: AuthnManagerImpl,
}

impl UserService<UserManagerImpl> {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            manager: UserManagerImpl::new(db_connection.clone()),
            authn_manager: AuthnManagerImpl::new(db_connection),
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

        let registered_user = self.manager.user_registration(new_user).await?;

        let response_data = registered_user.into();

        let data = Some(response_data);
        let response = RegisterUserResponse { data };
        Ok(Response::new(response))
    }

    async fn login_user(
        &self,
        request: Request<LoginUserRequest>,
    ) -> Result<Response<LoginUserResponse>, Status> {
        let login_user = LoginUser::from(request.into_inner());

        RequestValidator::new(&login_user).validate_for_response()?;

        let logged_in_user = self
            .manager
            .check_user_credentials(login_user.clone())
            .await?;

        let authn_token = self
            .authn_manager
            .generate_jwt_token(NewJwtParams {
                user_id: logged_in_user.id,
                device_id: login_user.device_id,
            })
            .await?;

        let response_data = authn_token.into();

        let data = Some(response_data);
        let response = LoginUserResponse { data };

        Ok(Response::new(response))
    }
}
