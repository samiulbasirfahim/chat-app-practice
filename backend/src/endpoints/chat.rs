use crate::service::chat_service;
use actix_web::web::{self, ServiceConfig};

pub fn register_endpoints(cfg: &mut ServiceConfig) {
    cfg.route("/chat", web::get().to(chat_service::chat_service));
}
