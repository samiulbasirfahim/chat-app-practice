use crate::utils::crypto::AuthUser;
use actix_web::web;
use sqlx::PgPool;

pub async fn send_email_verification(db: web::Data<PgPool>, user: AuthUser) {}
