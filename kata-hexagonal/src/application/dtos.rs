use std::fmt::Display;

use crate::domain::entities::user::UserDto;

#[derive(Clone)]
pub struct UserRegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserRegisterResponse {
    pub id: String,
    pub email: String,
}

impl Display for UserRegisterResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, email: {}", self.id, self.email)
    }
}

impl From<UserDto> for UserRegisterResponse {
    fn from(user: UserDto) -> Self {
        UserRegisterResponse {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserLoginResponse {
    pub id: String,
    pub email: String,
}

impl Display for UserLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, email: {}", self.id, self.email)
    }
}

impl From<UserDto> for UserLoginResponse {
    fn from(user: UserDto) -> Self {
        UserLoginResponse {
            id: user.id,
            email: user.email,
        }
    }
}
