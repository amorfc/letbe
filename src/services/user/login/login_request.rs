use validator::Validate;

use crate::services::proto::user::LoginUserRequest;

#[derive(Debug, Validate, Clone, Default)]
pub struct LoginUser {
    #[validate(email(message = "Please enter a valid email"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 30,
        message = "Pelease enter a password between 10 and 30 char"
    ))]
    pub password: String,
    #[validate(length(
        min = 5,
        max = 255,
        message = "Pelease enter a device id between 5 and 255 char"
    ))]
    pub device_id: String,
}

impl LoginUser {}

impl From<LoginUserRequest> for LoginUser {
    fn from(value: LoginUserRequest) -> Self {
        Self {
            email: value.email,
            password: value.password,
            device_id: value.device_id,
        }
    }
}
