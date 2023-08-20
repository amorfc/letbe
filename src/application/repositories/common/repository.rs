use anyhow::Result;
use migration::IntoCondition;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, TransactionTrait,
};

use crate::infra::db_initializor::LetDbTransaction;

#[tonic::async_trait]
pub trait RepositoryTrait<A, E>: DbConnectionProvider
where
    A: ActiveModelTrait + ActiveModelBehavior + Send + 'static,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    E: EntityTrait + 'static,
{
    async fn save(&self, model: A) -> Result<A> {
        let db_conn = self.db_connection();

        let res = model.save(db_conn).await?;

        Ok(res)
    }

    async fn insert(&self, model: A) -> Result<A> {
        let db_conn = self.db_connection();

        let res = model.insert(db_conn).await?;
        Ok(res.into_active_model())
    }

    async fn find_one<F>(&self, f: F) -> Result<Option<E::Model>>
    where
        F: IntoCondition + Send,
    {
        let db_conn = self.db_connection();

        let res = E::find().filter(f).one(db_conn).await?;
        Ok(res)
    }
}

pub trait DbConnectionProvider {
    fn db_connection(&self) -> &DatabaseConnection;
}

// Define a new trait that provides the db_transaction method
#[tonic::async_trait]
pub trait DbTransactionProvider: DbConnectionProvider {
    async fn db_tx(&self) -> Result<LetDbTransaction, DbErr> {
        self.db_connection().begin().await
    }
}
