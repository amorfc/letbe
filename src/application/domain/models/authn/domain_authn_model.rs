use entity::authn as AuthnEntity;

use sea_orm::prelude::DateTimeWithTimeZone;

#[derive(Clone, Debug)]
pub struct DomainAuthnModel {
    pub id: i32,
    pub user_id: i32,
    pub device_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expired_time: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub refreshed_at: Option<DateTimeWithTimeZone>,
    pub revoked_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

impl From<AuthnEntity::Model> for DomainAuthnModel {
    fn from(value: AuthnEntity::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            device_id: value.device_id,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expired_time: value.expired_time,
            refreshed_at: value.refreshed_at,
            revoked_at: value.revoked_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
