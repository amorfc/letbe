use entity::user as UserEntity;
use sea_orm::TryIntoModel;

use crate::{
    application::repositories::common::repository::{
        BaseRepositoryImpl, RepoDbConnectionProvider, RepoDbTransactionProvider,
    },
    infra::db_initializor::LetDbConnection,
};

// Define a new trait that provides the db_connection method

pub struct UserRepository {
    db_connection: LetDbConnection,
}

#[tonic::async_trait]
pub trait UserRepositoryTrait
where
    Self: BaseRepositoryImpl<UserEntity::ActiveModel, UserEntity::Entity>,
{
    fn new(db_connection: LetDbConnection) -> Self;
    async fn create_user(&self, user: UserEntity::ActiveModel)
        -> Result<UserEntity::Model, String>;
}

#[tonic::async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }

    async fn create_user(
        &self,
        active_model: UserEntity::ActiveModel,
    ) -> Result<UserEntity::Model, String> {
        let created_user = self.insert(active_model).await?;
        let created_user = created_user
            .try_into_model()
            .map_err(|_| "Failed to convert")?;

        Ok(created_user)
    }
}

// Then, when you implement the trait, you can specify the types:
#[tonic::async_trait]
impl BaseRepositoryImpl<UserEntity::ActiveModel, UserEntity::Entity> for UserRepository {}

impl RepoDbConnectionProvider for UserRepository {
    fn db_connection(&self) -> &LetDbConnection {
        &self.db_connection
    }
}

impl RepoDbTransactionProvider for UserRepository {}
