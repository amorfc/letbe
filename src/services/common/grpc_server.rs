pub trait LetGrpcServer<T> {
    fn serve(&self) -> T;
}
