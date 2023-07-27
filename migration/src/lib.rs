pub use entity::prelude::*;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20220101_000002_create_enum;
pub struct Migrator;

pub mod utils;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000002_create_enum::Migration),
            Box::new(m20220101_000001_create_user_table::Migration),
        ]
    }
}
