use entity::user as UserEntity;

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
    async fn create_user(&self, user: UserEntity::ActiveModel) -> Result<(), String>;
}

#[tonic::async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }

    async fn create_user(&self, model: UserEntity::ActiveModel) -> Result<(), String> {
        self.create(model).await?;

        Ok(())
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
