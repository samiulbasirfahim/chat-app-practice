use actix_web::web::ServiceConfig;

mod chat;
mod health;
mod user;

pub fn register_endpoints(cfg: &mut ServiceConfig) {
    health::register_endpoints(cfg);
    user::register_endpoints(cfg);
}
