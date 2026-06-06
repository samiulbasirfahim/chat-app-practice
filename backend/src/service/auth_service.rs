use crate::dtos::user::{
    LoginPayload, NewUser, UpdateUser, UsernamePayload, ValidateUpdateUserFields,
};
use crate::models::user::User;
use crate::utils::crypto::{AuthUser, compare_password, hash_password, tokens_generator};
use crate::utils::error_handler::build_error;
use crate::utils::random::generate_random_username;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use validator::Validate;

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
    if let Err(_) = password_match {
        return HttpResponse::InternalServerError().json(build_error("Internal Server Error"));
    }

    let is_logged_in = password_match.unwrap();

    if !is_logged_in {
        return HttpResponse::Unauthorized().json(build_error("Credentials missmatch"));
    };

    let tokens = match tokens_generator(user.id) {
        Ok(tokens) => tokens,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(build_error("Token Generation Failed"));
        }
    };

    HttpResponse::Ok().json(serde_json::json!(tokens))
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

pub async fn update_my_user_info_service(
    db: web::Data<PgPool>,
    user: AuthUser,
    form: MultipartForm<UpdateUser>,
) -> impl Responder {
    let validated_payload = ValidateUpdateUserFields {
        first_name: form.first_name.as_ref().map(|t| t.0.clone()),
        last_name: form.last_name.as_ref().map(|t| t.0.clone()),
    };

    println!("{:?}", user);

    if let Err(validation_err) = validated_payload.validate() {
        return HttpResponse::BadRequest().json(validation_err);
    }

    if let Some(avatar) = &form.avatar {
        // let res = upload_avatar(avatar)
    }

    HttpResponse::Ok().finish()
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
