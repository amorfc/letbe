use anyhow::bail;
use anyhow::{anyhow, Result};
use entity::club as ClubEntity;
use entity::user as UserEntity;
use sea_orm::ActiveValue;
use sea_orm::DatabaseTransaction;
use sea_orm::{entity::prelude::*, TransactionTrait};

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
};

// User Manager Trait that requires UserRepositoryTrait
#[tonic::async_trait]
pub trait ClubRepositoryTrait:
    RepositoryTrait<ClubEntity::ActiveModel, ClubEntity::Entity>
{
    async fn create_club(
        &self,
        club: ClubEntity::ActiveModel,
        club_owner_id: Option<i32>,
    ) -> Result<ClubEntity::Model> {
        let tx = self.db_connection().begin().await?;
        let club = club.insert(&tx).await?;

        if let Some(owner_id) = club_owner_id {
            self.update_user_club_id_tx(owner_id, club.id, &tx).await?;
        }

        tx.commit().await?;

        Ok(club)
    }
    async fn find_club_by_name(&self, name: &str) -> Result<Option<ClubEntity::Model>> {
        let res = self.find_one(ClubEntity::Column::Name.eq(name)).await?;

        Ok(res)
    }

    async fn update_user_club_id(&self, user_id: i32, club_id: i32) -> Result<UserEntity::Model> {
        let tx = self.db_connection().begin().await?;
        let res = self.update_user_club_id_tx(user_id, club_id, &tx).await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn update_user_club_id_tx(
        &self,
        user_id: i32,
        club_id: i32,
        tx: &DatabaseTransaction,
    ) -> Result<UserEntity::Model> {
        let user = UserEntity::Entity::find_by_id(user_id)
            .filter(UserEntity::Column::DeletedAt.is_null())
            .one(tx)
            .await?
            .ok_or(anyhow!("User not found"))?;

        if let Some(club_id) = user.club_id {
            bail!("User already has a club with id {}", club_id);
        }

        let mut user: UserEntity::ActiveModel = user.into();
        user.club_id = ActiveValue::Set(Some(club_id));

        let user = user.update(tx).await?;

        Ok(user)
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

    pub async fn find_user_club_by_id(&self, user_id: i32) -> Result<Option<ClubEntity::Model>> {
        let db = self.db_connection();

        let user = UserEntity::Entity::find_by_id(user_id)
            .filter(UserEntity::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(anyhow!("User not found"))?;

        let mut res = None;

        if let Some(club_id) = user.club_id {
            res = user
                .find_related(ClubEntity::Entity)
                .filter(ClubEntity::Column::Id.eq(club_id))
                .one(db)
                .await?;
        }

        Ok(res)
    }
}

impl DbConnectionProvider for ClubRepositoryImpl {
    fn db_connection(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<ClubEntity::ActiveModel, ClubEntity::Entity> for ClubRepositoryImpl {}
