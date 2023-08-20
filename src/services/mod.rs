pub mod common;
pub mod user;

pub(crate) mod proto {
    pub(crate) mod user {
        use tonic::{include_file_descriptor_set, include_proto};

        include_proto!("user");
        pub(crate) const USER_FILE_DESCRIPTOR_SET: &[u8] =
            include_file_descriptor_set!("user_descriptor");
    }
}
