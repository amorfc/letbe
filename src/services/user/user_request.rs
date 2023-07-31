use std::fmt::Display;

use entity::user as UserEntity;
use lazy_static::lazy_static;
use regex::Regex;
use sea_orm::ActiveValue;
use validator::Validate;

use crate::services::proto::user::RegisterUserRequest;

lazy_static! {
    static ref REGEX_USER_TYPE: Regex = Regex::new(r"^\s*(Individual|Corporation)\s*$").unwrap();
}

#[derive(Debug, Validate, Clone)]
pub struct NewUser {
    #[validate(email(message = "Please enter a valid email"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 30,
        message = "Pelease enter a password between 10 and 30 char"
    ))]
    pub password: String,

    #[validate(regex(path = "REGEX_USER_TYPE", message = "Please enter a valid user type"))]
    pub user_type: String,
    #[validate(length(
        min = 10,
        max = 30,
        message = "Pelease enter a name between 10 and 30 char"
    ))]
    pub name: String,
    #[validate(length(
        min = 10,
        max = 30,
        message = "Pelease enter a surname between 10 and 30 char"
    ))]
    pub surname: String,
}

pub enum RequestUserType {
    Individual,
    Corporation,
}

impl From<i32> for RequestUserType {
    fn from(value: i32) -> Self {
        match value {
            0 => RequestUserType::Individual,
            1 => RequestUserType::Corporation,
            _ => panic!("Invalid user type"),
        }
    }
}

impl From<String> for RequestUserType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Individual" => RequestUserType::Individual,
            "Corporation" => RequestUserType::Corporation,
            _ => panic!("Invalid user type"),
        }
    }
}

impl Display for RequestUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RequestUserType::Individual => "Individual",
            RequestUserType::Corporation => "Corporation",
        };
        write!(f, "{}", value)
    }
}

impl RequestUserType {
    pub fn to_int(&self) -> i32 {
        match self {
            RequestUserType::Individual => 0,
            RequestUserType::Corporation => 1,
        }
    }
}

impl From<RegisterUserRequest> for NewUser {
    fn from(value: RegisterUserRequest) -> Self {
        Self {
            password: value.password,
            email: value.email,
            user_type: RequestUserType::from(value.user_type).to_string(),
            name: value.name,
            surname: value.surname,
        }
    }
}

pub struct NewUserActiveModelWrapper(pub UserEntity::ActiveModel);

impl From<NewUser> for NewUserActiveModelWrapper {
    fn from(value: NewUser) -> Self {
        Self(UserEntity::ActiveModel {
            name: ActiveValue::set(value.name),
            surname: ActiveValue::set(value.surname),
            email: ActiveValue::set(value.email),
            password: ActiveValue::set(value.password),
            user_type: ActiveValue::set(UserEntity::UserType::from(value.user_type)),
            ..Default::default()
        })
    }
}
