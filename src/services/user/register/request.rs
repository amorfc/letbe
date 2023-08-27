use std::fmt::Display;

use anyhow::{anyhow, Result};
use entity::{sea_orm_active_enums::UserTypeEnum, user as UserEntity};
use lazy_static::lazy_static;
use regex::Regex;
use sea_orm::ActiveValue;
use validator::Validate;

use crate::{
    services::proto::user::RegisterUserRequest,
    shared::utils::{datetime::LettDate, hasher::LettHasher},
};

lazy_static! {
    static ref REGEX_USER_TYPE: Regex =
        Regex::new(r"^\s*(Corporation|Tutor|Member|Student|Guest|Other)\s*$").unwrap();
}

#[derive(Debug, Validate, Clone, Default)]
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

    // These fields are not validated
    pub salt: Option<String>,
    pub hashed_password: Option<String>,
}

impl NewUser {
    pub fn hash_password(&mut self) -> Result<()> {
        let hasher = LettHasher::hash_with_salt(&self.password)?;
        self.hashed_password = Some(hasher.hashed);
        self.salt = Some(hasher.salt);

        Ok(())
    }

    pub fn is_hashed(&self) -> bool {
        self.salt.is_some() && self.hashed_password.is_some()
    }
}

pub enum RequestUserType {
    Corporation,
    Tutor,
    Member,
    Student,
    Guest,
    Other,
}

impl From<i32> for RequestUserType {
    fn from(value: i32) -> Self {
        match value {
            0 => RequestUserType::Corporation,
            1 => RequestUserType::Tutor,
            2 => RequestUserType::Member,
            3 => RequestUserType::Student,
            4 => RequestUserType::Guest,
            _ => RequestUserType::Other,
        }
    }
}

impl From<String> for RequestUserType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Corporation" => RequestUserType::Corporation,
            "Tutor" => RequestUserType::Tutor,
            "Member" => RequestUserType::Member,
            "Student" => RequestUserType::Student,
            "Guest" => RequestUserType::Guest,
            _ => RequestUserType::Other,
        }
    }
}

impl Display for RequestUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RequestUserType::Corporation => "Corporation",
            RequestUserType::Tutor => "Tutor",
            RequestUserType::Member => "Member",
            RequestUserType::Student => "Student",
            RequestUserType::Guest => "Guest",
            RequestUserType::Other => "Other",
        };
        write!(f, "{}", value)
    }
}

impl RequestUserType {
    pub fn to_int(&self) -> i32 {
        match self {
            RequestUserType::Corporation => 0,
            RequestUserType::Tutor => 1,
            RequestUserType::Member => 2,
            RequestUserType::Student => 3,
            RequestUserType::Guest => 4,
            RequestUserType::Other => 5,
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
            ..Default::default()
        }
    }
}

pub struct NewUserActiveModelWrapper(pub UserEntity::ActiveModel);

impl TryFrom<NewUser> for NewUserActiveModelWrapper {
    type Error = anyhow::Error;

    fn try_from(mut value: NewUser) -> Result<Self, Self::Error> {
        value.hash_password()?;
        let password = value
            .hashed_password
            .ok_or(anyhow!("Password could not generated hashed"))?;

        let salt = value
            .salt
            .ok_or(anyhow!("Salt could not generated hashed"))?;

        let created_at = LettDate::now_dt_with_tz();

        Ok(Self(UserEntity::ActiveModel {
            name: ActiveValue::set(value.name),
            surname: ActiveValue::set(value.surname),
            email: ActiveValue::set(value.email),
            password: ActiveValue::set(password),
            salt: ActiveValue::set(salt),
            user_type: ActiveValue::set(UserTypeEnum::from(value.user_type)),
            created_at: ActiveValue::Set(created_at),
            ..Default::default()
        }))
    }
}
