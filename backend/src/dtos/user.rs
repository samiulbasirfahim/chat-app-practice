use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 2, message = "First name must be at least 2 charectars long"))]
    pub first_name: String,
    #[validate(length(min = 2, message = "First name must be at least 2 characters long"))]
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub struct LoginResponse {
    pub refresh: String,
    pub access: String,
}
