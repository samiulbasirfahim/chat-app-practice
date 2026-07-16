use crate::dtos::user::{UpdateUser, ValidateUpdateUserFields};
use crate::utils::crypto::AuthUser;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

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
