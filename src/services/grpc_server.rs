pub trait LettGrpcServer<T> {
    fn serve(&self) -> T;
}
