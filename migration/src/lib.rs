pub use entity::prelude::*;
pub use sea_orm_migration::prelude::*;

mod m20230814_192414_create_authn_table;
mod m20230822_121559_create_user_table;
pub struct Migrator;

pub mod utils;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230822_121559_create_user_table::Migration),
            Box::new(m20230814_192414_create_authn_table::Migration),
        ]
    }
}
