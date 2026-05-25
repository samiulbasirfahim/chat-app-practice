use crate::dtos::user::{LoginPayload, NewUser};
use crate::utils::crypto::hash_password;
use actix_web::{HttpResponse, Responder, web};
use actix_web_validator::Json;

pub async fn login_service(body: web::Json<LoginPayload>) -> impl Responder {
    "Logged in"
}

pub async fn register_service(payload: Json<NewUser>) -> impl Responder {
    let password = payload.password.clone();
    let blocking_result = web::block(move || hash_password(password)).await;
    let password_hash = match blocking_result {
        Ok(Ok(p)) => p,
        _ => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    println!("Hash: {}", password_hash);
    HttpResponse::Ok().body("Registration successful")
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

pub async fn check_username() -> impl Responder {
    "Username is available"
}
