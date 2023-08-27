use crate::{
    application::domain::models::authn::domain_authn_model::DomainAuthnModel,
    services::proto::user::LoggedInUserResponseData, shared::utils::datetime::LettDate,
};

impl From<DomainAuthnModel> for LoggedInUserResponseData {
    fn from(val: DomainAuthnModel) -> Self {
        Self {
            access_token: val.access_token,
            refresh_token: val.refresh_token,
            expires_at: LettDate::as_response_string(val.expired_time),
        }
    }
}
