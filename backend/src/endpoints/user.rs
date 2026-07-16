use crate::service::{auth_service, user::*};
use actix_web::web::{self, ServiceConfig};

pub fn register_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/login", web::post().to(login_service))
            .route("/register", web::post().to(register_service))
            .route("/check-username", web::get().to(check_username))
            .route("/set-username", web::post().to(set_username)), // .route(
                                                                   //     "/refresh",
                                                                   //     web::post().to(auth_service::refresh_token_service),
                                                                   // )
                                                                   // .route("/logout", web::post().to(auth_service::logout_service))
                                                                   // .route("/me", web::get().to(auth_service::get_my_user_info_service))
                                                                   // .route(
                                                                   //     "/me",
                                                                   //     web::patch().to(auth_service::update_my_user_info_service),
                                                                   // )
                                                                   // .route(
                                                                   //     "/change-password",
                                                                   //     web::post().to(auth_service::change_password_service),
                                                                   // )
                                                                   // .route(
                                                                   //     "/forgot-password",
                                                                   //     web::post().to(auth_service::forgot_password_service),
                                                                   // )
                                                                   // .route(
                                                                   //     "/reset-password",
                                                                   //     web::post().to(auth_service::reset_password_service),
                                                                   // )
                                                                   // .route(
                                                                   //     "/verify-email",
                                                                   //     web::post().to(auth_service::verify_email_service),
                                                                   // )
                                                                   // .route(
                                                                   //     "/resend-otp",
                                                                   //     web::post().to(auth_service::resend_otp_service),
                                                                   // )
                                                                   // .route("/check_username", web::post().to())
                                                                   // .route("/set_username", web::post().to(auth_service::set_username)),
    );
}
