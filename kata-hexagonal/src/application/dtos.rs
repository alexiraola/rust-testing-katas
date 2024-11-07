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

impl UserRegisterResponse {
    pub fn from(user: UserDto) -> Self {
        UserRegisterResponse {
            id: user.id,
            email: user.email,
        }
    }
}
