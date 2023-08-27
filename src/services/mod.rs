pub mod club;
pub mod common;
pub mod extensions;
pub mod middlewares;
pub mod user;

pub(crate) mod proto {
    use tonic::include_file_descriptor_set;

    pub(crate) const LETT_FILE_DESCRIPTOR_SET: &[u8] =
        include_file_descriptor_set!("lett_descriptor");

    pub(crate) mod user {
        use tonic::include_proto;
        include_proto!("user");
    }

    pub(crate) mod club {
        use tonic::include_proto;
        include_proto!("club");
    }
}
