// m20220101_000002_create_enum.rs

use std::vec;

use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Schema},
};

use entity::user::{self, Gender, UserType};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_enum" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let connection = manager.get_connection();
        let schema = Schema::new(db_postgres);
        let create_stms = vec![
            schema.create_enum_from_active_enum::<UserType>(),
            schema.create_enum_from_active_enum::<Gender>(),
        ];

        let stms = create_stms
            .iter()
            .map(|stm| db_postgres.build(stm))
            .collect::<Vec<_>>();

        for stm in stms {
            connection.execute(stm).await?;
        }

        Ok(())
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
