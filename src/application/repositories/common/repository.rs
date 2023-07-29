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
pub trait RepositoryImpl<A, E>: RepoDbConnectionProvider + RepoDbTransactionProvider
where
    E: EntityTrait,
    A: ActiveModelTrait + ActiveModelBehavior + Send + 'static,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    async fn save_as_commit(&self, db_tx: &LetDbConnection, model: A) -> Result<A, String> {
        match model.save(db_tx).await {
            Ok(model) => Ok(model),
            Err(_) => return Err("Failed to save model".to_string()),
        }
    }

    async fn insert_as_commit(
        &self,
        db_tx: &LetDbTransaction,
        model: A,
    ) -> Result<<A::Entity as EntityTrait>::Model, String> {
        match model.insert(db_tx).await {
            Ok(model) => Ok(model),
            Err(_) => return Err("Failed to insert model".to_string()),
        }
    }

    async fn create(&self, model: A) -> Result<(), String> {
        let txn = self
            .db_tx()
            .await
            .map_err(|_| "Failed to create transaction")?;

        let _ = &self.insert_as_commit(&txn, model).await?;

        match txn.commit().await {
            Ok(_) => (),
            Err(_) => return Err("Failed to commit model".to_string()),
        };

        Ok(())
    }
}
