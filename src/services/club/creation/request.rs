use entity::club as ClubEntity;
use sea_orm::ActiveValue;
use validator::Validate;

use crate::{services::proto::club::CreateClubRequest, shared::utils::datetime::LettDate};

#[derive(Debug, Validate, Clone, Default)]
pub struct NewClub {
    #[validate(length(
        min = 5,
        max = 100,
        message = "Pelease enter a valid club name between 5 and 100 char"
    ))]
    pub name: String,

    #[validate(length(
        min = 10,
        max = 100,
        message = "Pelease enter a validated legal club name between 10 and 100 char"
    ))]
    pub legal_name: String,
}

pub struct NewClubActiveModelWrapper(pub ClubEntity::ActiveModel);

impl From<NewClub> for NewClubActiveModelWrapper {
    fn from(value: NewClub) -> Self {
        let created_at = LettDate::now_dt_with_tz();

        Self(ClubEntity::ActiveModel {
            name: ActiveValue::set(value.name),
            legal_name: ActiveValue::set(value.legal_name),
            created_at: ActiveValue::Set(created_at),
            ..Default::default()
        })
    }
}

impl From<CreateClubRequest> for NewClub {
    fn from(value: CreateClubRequest) -> Self {
        Self {
            name: value.name,
            legal_name: value.legal_name,
        }
    }
}
