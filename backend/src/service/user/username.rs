use crate::dtos::user::UsernamePayload;
use crate::models::user::User;
use crate::utils::crypto::AuthUser;
use crate::utils::error_handler::build_error;
use crate::utils::succes_handler::success_generator;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;

pub async fn check_username(
    payload: web::Query<UsernamePayload>,
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

pub async fn set_username(
    payload: Json<UsernamePayload>,
    redis: web::Data<MultiplexedConnection>,
    db: web::Data<sqlx::PgPool>,
    user: AuthUser,
) -> impl Responder {
    let username = payload.username.clone();
    let prev_username = match User::get_by_id(user.user_id, db.get_ref()).await {
        Ok(Some(u)) => u.username,
        Ok(None) => {
            return HttpResponse::NotFound().json(build_error("User not found"));
        }
        Err(e) => {
            eprintln!("DB Error fetching user: {}", e);
            return HttpResponse::InternalServerError().json(build_error("Database Error"));
        }
    };

    match User::update_username(user.user_id, &username, &db).await {
        Ok(_) => (),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return HttpResponse::Conflict().json(build_error("Username is already taken"));
                }
            }

            eprintln!("DB Error mutating user: {}", e);
            return HttpResponse::InternalServerError().json(build_error("Database Error"));
        }
    };

    let mut redcon = redis.get_ref().clone();
    let _ = redcon
        .set::<_, _, ()>(format!("username:{}", username), true)
        .await;

    let _ = redcon
        .set::<_, _, ()>(format!("username:{}", prev_username), false)
        .await;

    return success_generator("Username updates succesfully.");
}
