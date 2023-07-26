use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DbErr, Statement};

use crate::{config::ENV_CONFIG, LetDbConnection};

#[tonic::async_trait]
pub trait DatabaseInitializerImpl {
    type ConnType;
    type ErrorType;
    type ConnOptType;

    fn db_url() -> String;
    fn connection_opt(db_url: String) -> Self::ConnOptType;
    async fn connect() -> Result<Self::ConnType, Self::ErrorType>;
    async fn create_connect_db(
        db_name: String,
        db: &Self::ConnType,
    ) -> Result<Self::ConnType, Self::ErrorType>;
}
pub struct DatabaseInitializer {}

#[tonic::async_trait]
impl DatabaseInitializerImpl for DatabaseInitializer {
    type ConnType = LetDbConnection;
    type ConnOptType = ConnectOptions;
    type ErrorType = DbErr;

    async fn connect() -> Result<Self::ConnType, Self::ErrorType> {
        let opt = Self::connection_opt(Self::db_url());

        let db_conn = Database::connect(opt).await?;
        let db_conn =
            DatabaseInitializer::create_connect_db(ENV_CONFIG.db_name.to_owned(), &db_conn).await?;

        Ok(db_conn)
    }

    fn db_url() -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            ENV_CONFIG.postgres_user,
            ENV_CONFIG.postgres_password,
            ENV_CONFIG.db,
            ENV_CONFIG.db_port,
        )
    }

    fn connection_opt(db_url: String) -> Self::ConnOptType {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(30))
            .sqlx_logging(true)
            .to_owned()
    }

    async fn create_connect_db(
        db_name: String,
        db: &Self::ConnType,
    ) -> Result<Self::ConnType, Self::ErrorType> {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE \"{}\";", db_name),
        ))
        .await?;

        let database_url = format!("{}/{}", Self::db_url(), db_name);
        let opt = Self::connection_opt(database_url);

        let db_conn = Database::connect(opt).await?;
        Ok(db_conn)
    }
}
