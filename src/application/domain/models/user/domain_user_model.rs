use anyhow::{bail, Result};
use entity::{sea_orm_active_enums::UserTypeEnum, user as UserEntity};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::TryIntoModel;

use crate::shared::utils::hasher::LettHasher;

pub struct DomainUserModel {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub user_type: DomainUserType,
    pub club_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl DomainUserModel {
    pub fn verify_password(&self, password: &str) -> Result<Option<()>> {
        let verified = LettHasher::verify_password(password, &self.password)?;
        if !verified {
            bail!("Password is incorrect")
        }

        Ok(Some(()))
    }
}

impl From<UserEntity::Model> for DomainUserModel {
    fn from(value: UserEntity::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            surname: value.surname,
            email: value.email,
            password: value.password,
            user_type: DomainUserType::from(value.user_type),
            club_id: value.club_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl From<UserEntity::ActiveModel> for DomainUserModel {
    fn from(value: UserEntity::ActiveModel) -> Self {
        let value = value.try_into_model().unwrap();
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            password: value.password,
            surname: value.surname,
            user_type: value.user_type.into(),
            club_id: value.club_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

pub enum DomainUserType {
    Corporation,
    Tutor,
    Member,
    Student,
    Guest,
    Other,
}

impl From<UserTypeEnum> for DomainUserType {
    fn from(value: UserTypeEnum) -> Self {
        match value {
            UserTypeEnum::Corporation => DomainUserType::Corporation,
            UserTypeEnum::Tutor => DomainUserType::Tutor,
            UserTypeEnum::Member => DomainUserType::Member,
            UserTypeEnum::Student => DomainUserType::Student,
            UserTypeEnum::Guest => DomainUserType::Guest,
            UserTypeEnum::Other => DomainUserType::Other,
        }
    }
}
