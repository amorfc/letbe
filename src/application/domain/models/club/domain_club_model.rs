use entity::club as ClubEntity;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::TryIntoModel;

pub struct DomainClubModel {
    pub id: i32,
    pub name: String,
    pub legal_name: String,
    pub created_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

impl DomainClubModel {}
impl From<ClubEntity::Model> for DomainClubModel {
    fn from(value: ClubEntity::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            legal_name: value.legal_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl From<ClubEntity::ActiveModel> for DomainClubModel {
    fn from(value: ClubEntity::ActiveModel) -> Self {
        let value = value.try_into_model().unwrap();
        Self {
            id: value.id,
            name: value.name,
            legal_name: value.legal_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}
