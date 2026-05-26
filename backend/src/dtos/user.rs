use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

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

pub struct TokenResponse {
    pub refresh: String,
    pub access: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UsernamePayload {
    #[validate(
        length(
            min = 3,
            max = 20,
            message = "Username must be between 3 and 20 characters"
        ),
        regex(
            path = "USERNAME_REGEX",
            message = "Username can only contain letters, numbers, and underscores (_)"
        )
    )]
    pub username: String,
}
