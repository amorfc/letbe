use entity::user as UserEntity;

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
};

// User Manager Trait that requires UserRepositoryTrait
#[tonic::async_trait]
pub trait UserRepositoryTrait:
    RepositoryTrait<UserEntity::ActiveModel, UserEntity::Entity>
{
    async fn create_user(
        &self,
        user: UserEntity::ActiveModel,
    ) -> Result<UserEntity::ActiveModel, String> {
        let a = self.save(user).await?;
        Ok(a)
    }
    // Define methods specific to UserRepository here
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
    fn db_connection(&self) -> &LetDbConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<UserEntity::ActiveModel, UserEntity::Entity> for UserRepositoryImpl {}
