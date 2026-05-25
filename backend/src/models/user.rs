use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub is_verified: bool,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

