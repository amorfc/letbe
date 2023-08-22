use sea_orm_migration::prelude::*;

use crate::m20230822_121559_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Authn::Table)
                    .col(
                        ColumnDef::new(Authn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Authn::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_authn_user_id")
                            .from(Authn::Table, Authn::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Authn::AccessToken).string().not_null())
                    .col(ColumnDef::new(Authn::RefreshToken).string().not_null())
                    .col(
                        ColumnDef::new(Authn::ExpiredTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Authn::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Authn::RefreshedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Authn::RevokedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Authn::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Authn::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Authn {
    Table,
    Id,
    UserId,
    AccessToken,
    RefreshToken,
    ExpiredTime,
    CreatedAt,
    RefreshedAt,
    RevokedAt,
    UpdatedAt,
}
