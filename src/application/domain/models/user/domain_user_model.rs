use anyhow::{bail, Result};
use entity::{sea_orm_active_enums::UserTypeEnum, user as UserEntity};

use crate::{
    services::proto::user::{RegisteredUserResponseData, UserType as ResponseUserType},
    shared::utils::hasher::LettHasher,
};

pub struct DomainUserModel {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub user_type: DomainUserType,
    // pub created_at: String,
    // pub updated_at: String,
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

impl From<DomainUserType> for ResponseUserType {
    fn from(value: DomainUserType) -> Self {
        match value {
            DomainUserType::Corporation => ResponseUserType::Corporation,
            DomainUserType::Tutor => ResponseUserType::Tutor,
            DomainUserType::Member => ResponseUserType::Member,
            DomainUserType::Student => ResponseUserType::Student,
            DomainUserType::Guest => ResponseUserType::Guest,
            _ => ResponseUserType::Other,
        }
    }
}

impl From<DomainUserModel> for RegisteredUserResponseData {
    fn from(val: DomainUserModel) -> Self {
        RegisteredUserResponseData {
            id: val.id,
            name: val.name,
            surname: val.surname,
            email: val.email,
            user_type: ResponseUserType::from(val.user_type).into(),
        }
    }
}
