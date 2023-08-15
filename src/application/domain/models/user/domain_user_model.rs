use entity::user as UserEntity;

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
    pub fn verify_password(&self, password: &str) -> Result<Option<()>, String> {
        let verified = LettHasher::verify_password(password, &self.password)?;
        if !verified {
            return Err("Password is incorrect".to_string());
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
    Individual,
    Corporation,
}

impl From<UserEntity::UserTypeEnum> for DomainUserType {
    fn from(value: UserEntity::UserTypeEnum) -> Self {
        match value {
            UserEntity::UserTypeEnum::Individual => DomainUserType::Individual,
            UserEntity::UserTypeEnum::Corporation => DomainUserType::Corporation,
        }
    }
}

impl From<DomainUserType> for ResponseUserType {
    fn from(value: DomainUserType) -> Self {
        match value {
            DomainUserType::Individual => ResponseUserType::Individual,
            DomainUserType::Corporation => ResponseUserType::Corporation,
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
