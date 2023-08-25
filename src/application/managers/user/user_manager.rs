use anyhow::{anyhow, bail, Result};

use crate::{
    application::{
        domain::models::user::domain_user_model::DomainUserModel,
        managers::common::manager::ManagerTrait,
        repositories::user::user_repository::{UserRepositoryImpl, UserRepositoryTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::{
        common::response::response_status::LettResError,
        user::{
            login::request::LoginUser,
            register::request::{NewUser, NewUserActiveModelWrapper},
        },
    },
};

#[tonic::async_trait]
pub trait UserManagerTrait: ManagerTrait<DomainUserModel> {
    async fn user_registration(
        &self,
        input_create_user: NewUser,
    ) -> Result<DomainUserModel, LettResError>;
    async fn check_user_credentials(
        &self,
        input_login_user: LoginUser,
    ) -> Result<DomainUserModel, LettResError>;
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

    pub async fn check_email_availability(&self, email: &str) -> Result<()> {
        let find_user = self.repo.find_user_by_email(email).await?;
        if let Some(exists_user) = find_user {
            bail!("User with email {} already exists", exists_user.email);
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl UserManagerTrait for UserManagerImpl {
    async fn user_registration(&self, new_user: NewUser) -> Result<DomainUserModel, LettResError> {
        self.check_email_availability(&new_user.email).await?;

        let active_model_wrapper: NewUserActiveModelWrapper = new_user.try_into()?;

        let created_user = self.repo.create_user(active_model_wrapper.0).await?;

        Ok(created_user.into())
    }

    async fn check_user_credentials(
        &self,
        login_user: LoginUser,
    ) -> Result<DomainUserModel, LettResError> {
        let find_user = self
            .repo
            .find_user_by_email(&login_user.email)
            .await?
            .ok_or(anyhow!("User not found"))?;

        let domain_user: DomainUserModel = find_user.into();
        domain_user.verify_password(&login_user.password)?;

        Ok(domain_user)
    }
}

impl ManagerTrait<DomainUserModel> for UserManagerImpl {}
