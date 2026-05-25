use actix_web::web::ServiceConfig;

mod auth;
mod health;
mod chat;

pub fn register_endpoints(cfg: &mut ServiceConfig) {
    health::register_endpoints(cfg);
    auth::register_endpoints(cfg);
}
