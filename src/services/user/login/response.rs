use crate::{
    application::domain::models::authn::domain_authn_model::DomainAuthnModel,
    services::proto::user::LoggedInUserResponseData,
    shared::utils::proto::time::date_time_utc_to_prost_timestamp,
};

impl From<DomainAuthnModel> for LoggedInUserResponseData {
    fn from(val: DomainAuthnModel) -> Self {
        Self {
            access_token: val.access_token,
            refresh_token: val.refresh_token,
            expires_at: Some(date_time_utc_to_prost_timestamp(val.expired_time)),
        }
    }
}
