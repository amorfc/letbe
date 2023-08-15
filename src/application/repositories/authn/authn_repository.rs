use entity::authn as AuthnEntity;
use sea_orm::{entity::prelude::*, ActiveValue};

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
    shared::utils::datetime::LettDate,
};

// User Manager Trait that requires AuthnRepositoryTrait
#[tonic::async_trait]
pub trait AuthnRepositoryTrait:
    RepositoryTrait<AuthnEntity::ActiveModel, AuthnEntity::Entity>
{
    async fn create_authn_token(
        &self,
        mut new_authn: AuthnEntity::ActiveModel,
    ) -> Result<AuthnEntity::ActiveModel, String> {
        new_authn.created_at = ActiveValue::Set(LettDate::now_dt_with_tz());

        let authn_token = self.save(new_authn).await?;
        Ok(authn_token)
    }
}

// Implementation of AuthnRepositoryTrait
pub struct AuthnRepositoryImpl {
    db_connection: LetDbConnection,
}

#[tonic::async_trait]
impl AuthnRepositoryTrait for AuthnRepositoryImpl {}

impl AuthnRepositoryImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }
}

impl DbConnectionProvider for AuthnRepositoryImpl {
    fn db_connection(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<AuthnEntity::ActiveModel, AuthnEntity::Entity> for AuthnRepositoryImpl {}
