use entity::authn;
use sea_orm_migration::prelude::*;

use crate::utils::migrator_utils;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let (db_postgres, connection, schema) = migrator_utils(manager);

        let table_create_stm = db_postgres.build(&schema.create_table_from_entity(authn::Entity));
        connection.execute(table_create_stm).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(authn::Entity).to_owned())
            .await
    }
}
