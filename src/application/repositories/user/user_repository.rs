use anyhow::Result;
use entity::user::{self as UserEntity, Column};
use sea_orm::entity::prelude::*;

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
};

// User Manager Trait that requires UserRepositoryTrait
#[tonic::async_trait]
pub trait UserRepositoryTrait:
    RepositoryTrait<UserEntity::ActiveModel, UserEntity::Entity>
{
    async fn create_user(&self, user: UserEntity::ActiveModel) -> Result<UserEntity::ActiveModel> {
        let a = self.save(user).await?;
        Ok(a)
    }
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserEntity::Model>> {
        let res = self.find_one(UserEntity::Column::Email.eq(email)).await?;

        Ok(res)
    }
    async fn find_user_by_id(&self, id: i32) -> Result<Option<UserEntity::Model>> {
        let res = self.find_one(Column::Id.eq(id)).await?;

        Ok(res)
    }
}

// Implementation of UserRepositoryTrait
pub struct UserRepositoryImpl {
    db_connection: LetDbConnection,
}

#[tonic::async_trait]
impl UserRepositoryTrait for UserRepositoryImpl {}

impl UserRepositoryImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }
}

impl DbConnectionProvider for UserRepositoryImpl {
    fn db_connection(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<UserEntity::ActiveModel, UserEntity::Entity> for UserRepositoryImpl {}
