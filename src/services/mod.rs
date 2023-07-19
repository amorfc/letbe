pub mod base;
pub mod grpc_server;
pub mod user;

mod proto {
    pub(crate) mod user {
        use tonic::{include_file_descriptor_set, include_proto};

        include_proto!("user");
        pub(crate) const USER_FILE_DESCRIPTOR_SET: &[u8] =
            include_file_descriptor_set!("user_descriptor");
    }
}