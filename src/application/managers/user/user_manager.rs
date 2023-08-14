use entity::user;
use sea_orm::TryIntoModel;

use crate::{
    application::{
        domain::models::user::domain_user_model::DomainUserModel,
        managers::common::manager::ManagerTrait,
        repositories::user::user_repository::{UserRepositoryImpl, UserRepositoryTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::user::register::register_request::{NewUser, NewUserActiveModelWrapper},
};

#[tonic::async_trait]
pub trait UserManagerTrait: ManagerTrait<DomainUserModel> {
    async fn user_registration(
        &self,
        input_create_user: NewUser,
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

    pub async fn check_email_availability(&self, email: &str) -> Result<(), String> {
        let find_user = self.repo.find_user_by_email(email.into()).await?;
        if let Some(exists_user) = find_user {
            return Err(format!(
                "User with email ${} already exists",
                exists_user.email
            ));
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl UserManagerTrait for UserManagerImpl {
    async fn user_registration(&self, new_user: NewUser) -> Result<DomainUserModel, String> {
        self.check_email_availability(&new_user.email).await?;

        let active_model_wrapper: NewUserActiveModelWrapper = new_user
            .try_into()
            .map_err(|err| format!("Internal Server Error while registration, Error while converting to active model: {}", err))?;

        let created_user = self.repo.create_user(active_model_wrapper.0).await?;

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
