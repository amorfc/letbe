use sea_orm::DatabaseConnection;

pub trait LetGrpcServer<T> {
    fn serve(&self, db_connection: DatabaseConnection) -> T;
}
