// m20220101_000003_create_user_table.rs

use sea_orm_migration::prelude::*;

use entity::user::{self};

use crate::utils::migrator_utils;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let (db_postgres, connection, schema) = migrator_utils(manager);

        let table_create_stm = db_postgres.build(
            &schema
                .create_table_from_entity(user::Entity)
                .if_not_exists()
                .to_owned(),
        );
        connection.execute(table_create_stm).await?;

        Ok(())
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}

impl Migration {}
