use anyhow::Result;
use tonic::Request;

use crate::services::common::response::response_status::LettResError;

use super::user_context::UserContextExt;

pub struct ExtensionExtractor {}

impl ExtensionExtractor {
    pub fn auth_user_ext<T>(req: &Request<T>) -> Result<UserContextExt, LettResError> {
        req.extensions()
            .get::<UserContextExt>()
            .cloned()
            .ok_or(LettResError::Unauthenticated {
                message: "".to_string(),
            })
    }
}
