use crate::{
    application::domain::models::user::domain_user_model::{DomainUserModel, DomainUserType},
    services::proto::user::{RegisteredUserResponseData, UserType as ResponseUserType},
};

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
