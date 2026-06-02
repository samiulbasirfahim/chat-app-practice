use crate::dtos::user::{LoginPayload, NewUser, UsernamePayload};
use crate::models::user::User;
use crate::utils::crypto::{compare_password, hash_password};
use crate::utils::error_handler::build_error;
use crate::utils::random::generate_random_username;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use sqlx::PgPool;

pub async fn login_service(body: web::Json<LoginPayload>, db: web::Data<PgPool>) -> impl Responder {
    let user = User::get_by_username_email(&body.username, &db).await;
    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(build_error("No user found with given credentials"));
        }
        _ => {
            return HttpResponse::InternalServerError().json(build_error("Database Error"));
        }
    };

    let password_match = compare_password(&body.password, &user.password_hash);

    if let Ok(is_logged_in) = password_match {
        if is_logged_in {
            return HttpResponse::Ok().json(user);
        } else {
            return HttpResponse::Unauthorized().json(build_error("Credentials missmatch"));
        }
    } else {
        return HttpResponse::InternalServerError().json(build_error("Internal Server Error"));
    }
}

pub async fn register_service(payload: Json<NewUser>, db: web::Data<PgPool>) -> impl Responder {
    let password = payload.password.clone();
    let blocking_result = web::block(move || hash_password(password)).await;
    let password_hash = match blocking_result {
        Ok(Ok(p)) => p,
        _ => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    let username = generate_random_username();
    let user = User::create(&db, &payload, &password_hash, &username).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            HttpResponse::Conflict().json(build_error("Email or username already in use"))
        }
        Err(e) => {
            eprintln!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(build_error("Databse Error"))
        }
    }
}

pub async fn logout_service() -> impl Responder {
    "Logout successful"
}

pub async fn refresh_token_service() -> impl Responder {
    "Token refreshed"
}

pub async fn get_my_user_info_service() -> impl Responder {
    "User info retrieved"
}

pub async fn update_my_user_info_service() -> impl Responder {
    "User info updated"
}

pub async fn change_password_service() -> impl Responder {
    "Password changed"
}

pub async fn forgot_password_service() -> impl Responder {
    "Password reset link sent"
}

pub async fn reset_password_service() -> impl Responder {
    "Password reset successful"
}

pub async fn verify_email_service() -> impl Responder {
    "Email verified"
}

pub async fn resend_otp_service() -> impl Responder {
    "OTP resent"
}

pub async fn check_username(
    payload: Json<UsernamePayload>,
    redis: web::Data<MultiplexedConnection>,
) -> impl Responder {
    let username = payload.username.clone().to_lowercase();
    let username_to_check = format!("username:{username}");
    let mut conn = redis.get_ref().clone();
    let exist: bool = match conn.exists(username_to_check).await {
        Ok(exist) => exist,
        Err(_) => return HttpResponse::InternalServerError().json(build_error("REDIS Error")),
    };

    HttpResponse::Ok().json(serde_json::json!({
        "available": !exist
    }))
}

pub async fn set_username() -> impl Responder {
    "HELLo"
}
