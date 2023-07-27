use validator::Validate;

use crate::services::proto::user::RegisterUserRequest;

#[derive(Debug, Validate, Clone)]
pub struct RequestUser {
    #[validate(email(message = "Please enter a valid email"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 30,
        message = "Pelease enter a password between 10 and 30 char"
    ))]
    pub password: String,
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
