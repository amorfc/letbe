use crate::application::domain::models::authn::domain_authn_model::DomainAuthnModel;

#[derive(Debug, Clone, Default)]
pub struct UserContextExt {
    pub user_id: i32,
    pub device_id: String,
}

impl From<DomainAuthnModel> for UserContextExt {
    fn from(value: DomainAuthnModel) -> Self {
        Self {
            user_id: value.user_id,
            device_id: value.device_id,
        }
    }
}
