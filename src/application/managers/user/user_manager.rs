use entity::user;
use sea_orm::TryIntoModel;

use crate::{
    application::{
        domain::models::user::domain_user_model::DomainUserModel,
        managers::common::manager::ManagerTrait,
        repositories::user::user_repository::{UserRepositoryImpl, UserRepositoryTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::user::user_request::NewUserActiveModelWrapper,
};

#[tonic::async_trait]
pub trait UserManagerTrait: ManagerTrait<DomainUserModel> {
    async fn user_registration(
        &self,
        input_create_user: NewUserActiveModelWrapper,
    ) -> Result<DomainUserModel, String>;
}

// Implementation of UserManagerTrait
pub struct UserManagerImpl {
    repo: UserRepositoryImpl,
}

impl UserManagerImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            repo: UserRepositoryImpl::new(db_connection),
        }
    }
}

#[tonic::async_trait]
impl UserManagerTrait for UserManagerImpl {
    async fn user_registration(
        &self,
        new_user: NewUserActiveModelWrapper,
    ) -> Result<DomainUserModel, String> {
        let created_user = self.repo.create_user(new_user.0).await?;

        Ok(created_user.into())
    }
}

impl From<user::ActiveModel> for DomainUserModel {
    fn from(value: user::ActiveModel) -> Self {
        let value = value.try_into_model().unwrap();
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            password: value.password,
            surname: value.surname,
            user_type: value.user_type.into(),
        }
    }
}

impl ManagerTrait<DomainUserModel> for UserManagerImpl {}
