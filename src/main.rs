use tonic::transport::Server;

use crate::user::USER_FILE_DESCRIPTOR_SET;
pub mod entities;

mod user {
    use tonic::{include_file_descriptor_set, include_proto};

    include_proto!("user");
    pub(crate) const USER_FILE_DESCRIPTOR_SET: &[u8] =
        include_file_descriptor_set!("user_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(USER_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = "[::1]:50055";

    println!("Hello, Lett runs on {}!", addr);

    Server::builder()
        .add_service(reflection_service)
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
