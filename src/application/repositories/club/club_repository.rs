use anyhow::Result;
use entity::club as ClubEntity;
use sea_orm::entity::prelude::*;

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
};

// User Manager Trait that requires UserRepositoryTrait
#[tonic::async_trait]
pub trait ClubRepositoryTrait:
    RepositoryTrait<ClubEntity::ActiveModel, ClubEntity::Entity>
{
    async fn create_club(&self, user: ClubEntity::ActiveModel) -> Result<ClubEntity::ActiveModel> {
        let a = self.save(user).await?;
        Ok(a)
    }
    async fn find_club_by_name(&self, name: &str) -> Result<Option<ClubEntity::Model>> {
        let res = self.find_one(ClubEntity::Column::Name.eq(name)).await?;

        Ok(res)
    }
}

// Implementation of UserRepositoryTrait
pub struct ClubRepositoryImpl {
    db_connection: LetDbConnection,
}

#[tonic::async_trait]
impl ClubRepositoryTrait for ClubRepositoryImpl {}

impl ClubRepositoryImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }
}

impl DbConnectionProvider for ClubRepositoryImpl {
    fn db_connection(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<ClubEntity::ActiveModel, ClubEntity::Entity> for ClubRepositoryImpl {}
