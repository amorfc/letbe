use crate::LetDbConnection;

pub trait LetGrpcServer<T> {
    fn serve(&self, db_conn: LetDbConnection) -> T;
}
