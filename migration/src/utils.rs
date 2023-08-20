use sea_orm_migration::{
    sea_orm::{DbBackend, Schema},
    SchemaManager, SchemaManagerConnection,
};

pub fn migrator_utils<'a>(
    manager: &'a SchemaManager<'a>,
) -> (DbBackend, &SchemaManagerConnection<'a>, Schema) {
    let db_postgres = DbBackend::Postgres;
    let connection = manager.get_connection();
    let schema = Schema::new(db_postgres);

    (db_postgres, connection, schema)
}
