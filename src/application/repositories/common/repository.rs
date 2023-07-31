use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, TransactionTrait,
};

use crate::infra::db_initializor::{LetDbConnection, LetDbTransaction};

pub trait RepoDbConnectionProvider {
    fn db_connection(&self) -> &LetDbConnection;
}

// Define a new trait that provides the db_transaction method
#[tonic::async_trait]
pub trait RepoDbTransactionProvider: RepoDbConnectionProvider {
    async fn db_tx(&self) -> Result<LetDbTransaction, DbErr> {
        self.db_connection().begin().await
    }
}

#[tonic::async_trait]
pub trait BaseRepositoryImpl<A, E>: RepoDbConnectionProvider + RepoDbTransactionProvider
where
    A: ActiveModelTrait + ActiveModelBehavior + Send + 'static,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    E: EntityTrait + 'static,
{
    async fn save(&self, model: A) -> Result<A, String> {
        let db_conn = self.db_connection();

        match model.save(db_conn).await {
            Ok(model) => Ok(model),
            Err(_) => return Err("Failed to save model".to_string()),
        }
    }

    async fn insert(&self, model: A) -> Result<A, String> {
        let db_conn = self.db_connection();

        match model.insert(db_conn).await {
            Ok(model) => Ok(model.into_active_model()),
            Err(_) => return Err("Failed to insert model".to_string()),
        }
    }
}
