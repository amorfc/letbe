use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Club::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Club::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Club::Name).string().not_null())
                    .col(ColumnDef::new(Club::LegalName).string().not_null())
                    .col(ColumnDef::new(Club::LocationId).integer().null())
                    .col(
                        ColumnDef::new(Club::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Club::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Club::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Club::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Club {
    Table,
    Id,
    Name,
    LegalName,
    LocationId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
