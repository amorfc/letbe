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
        message = "Pelease enter a name between 10 and 30 char"
    ))]
    pub surname: String,
}

impl From<RegisterUserRequest> for RequestUser {
    fn from(value: RegisterUserRequest) -> Self {
        Self {
            password: value.password,
            email: value.email,
            user_type: value.user_type,
            name: value.name,
            surname: value.surname,
        }
    }
}

impl From<RequestUser> for RegisterUserRequest {
    fn from(value: RequestUser) -> Self {
        Self {
            user_type: value.user_type,
            password: value.password,
            name: value.name,
            surname: value.surname,
            email: value.email,
        }
    }
}
