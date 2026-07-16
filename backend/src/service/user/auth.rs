use crate::dtos::user::{LoginPayload, NewUser};
use crate::models::user::User;
use crate::utils::crypto::{compare_password, hash_password, tokens_generator};
use crate::utils::error_handler::build_error;
use crate::utils::random::generate_random_username;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
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
