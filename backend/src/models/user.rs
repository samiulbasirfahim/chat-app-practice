use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;

use crate::dtos::user::NewUser;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub username: String,
    pub is_verified: bool,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn create(
        db: &PgPool,
        payload: &NewUser,
        hashed_password: &str,
        username: &str,
    ) -> Result<User, sqlx::Error> {
        Ok(sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (first_name, last_name, email, password_hash, username)
                VALUES($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            payload.first_name,
            payload.last_name,
            payload.email,
            hashed_password,
            username
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn get_by_id(id: i32, db: &PgPool) -> Result<Option<User>, sqlx::Error> {
        Ok(sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?)
    }

    pub async fn get_by_username_email(
        username_email: &str,
        db: &PgPool,
    ) -> Result<Option<User>, sqlx::Error> {
        Ok(sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1 OR email = $1
            "#,
            username_email
        )
        .fetch_optional(db)
        .await?)
    }
    pub async fn update_username(
        id: i32,
        new_username: &str,
        db: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET username = $2
            WHERE id = $1
            "#,
            id,
            new_username
        )
        .execute(db)
        .await?;

        return Ok(());
    }
}
