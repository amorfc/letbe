use entity::user as UserEntity;

use crate::{
    application::repositories::common::repository::{
        RepoDbConnectionProvider, RepoDbTransactionProvider, RepositoryImpl,
    },
    infra::db_initializor::LetDbConnection,
};

// Define a new trait that provides the db_connection method

pub struct UserRepository {
    db_connection: LetDbConnection,
}

impl UserRepository {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }

    pub async fn create_user(&self, model: UserEntity::ActiveModel) -> Result<(), String> {
        self.create(model).await?;

        Ok(())
    }
}

// Then, when you implement the trait, you can specify the types:
#[tonic::async_trait]
impl RepositoryImpl<UserEntity::ActiveModel, UserEntity::Entity> for UserRepository {}

impl RepoDbConnectionProvider for UserRepository {
    fn db_connection(&self) -> &LetDbConnection {
        &self.db_connection
    }
}

impl RepoDbTransactionProvider for UserRepository {}
