use crate::infra::db_initializor::LetDbConnection;

pub trait LetGrpcServer<T> {
    fn serve(&self, db_connection: LetDbConnection) -> T;
}
