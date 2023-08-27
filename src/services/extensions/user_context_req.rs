use hyper::Uri;

use crate::application::domain::models::authn::domain_authn_model::DomainAuthnModel;

#[derive(Debug, Clone, Default)]
pub struct UserGrpcReqExt {
    pub uri: Uri,
    pub user_context: Option<UserContext>,
}

#[derive(Debug, Clone, Default)]
pub struct UserContext {
    pub user_id: i32,
    pub device_id: String,
}

impl From<DomainAuthnModel> for UserContext {
    fn from(value: DomainAuthnModel) -> Self {
        Self {
            user_id: value.user_id,
            device_id: value.device_id,
        }
    }
}
