use entity::authn as AuthnEntity;
use sea_orm::entity::prelude::*;

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
};

// User Manager Trait that requires AuthnRepositoryTrait
#[tonic::async_trait]
pub trait AuthnRepositoryTrait:
    RepositoryTrait<AuthnEntity::ActiveModel, AuthnEntity::Entity>
{
    async fn create_authn_token(
        &self,
        authn_token: AuthnEntity::ActiveModel,
    ) -> Result<AuthnEntity::ActiveModel, String> {
        let authn_token = self.save(authn_token).await?;
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
