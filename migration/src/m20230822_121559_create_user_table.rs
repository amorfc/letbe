// m20220101_000003_create_user_table.rs

use sea_orm_migration::{
    prelude::*,
    sea_orm::{DeriveMigrationName, EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

use crate::m20230822_144020_create_club_table::Club;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(UserType::Table).if_exists().to_owned())
            .await?;

        //Create UserType Enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(UserType::Table)
                    .values(UserType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Salt).string().not_null())
                    .col(
                        ColumnDef::new(User::UserType)
                            .not_null()
                            .enumeration(UserType::Table, UserType::iter().skip(1)),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Surname).string().not_null())
                    .col(ColumnDef::new(User::ClubId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_club_id")
                            .from(User::Table, User::ClubId)
                            .to(Club::Table, Club::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(User::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

impl Migration {}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    Salt,
    UserType,
    Name,
    Surname,
    ClubId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden, EnumIter)]
pub enum UserType {
    Table,
    #[iden = "Corporation"]
    Corporation,
    #[iden = "Tutor"]
    Tutor,
    #[iden = "Member"]
    Member,
    #[iden = "Student"]
    Student,
    #[iden = "Guest"]
    Guest,
    #[iden = "Other"]
    Other,
}
