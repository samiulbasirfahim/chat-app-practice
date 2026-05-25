use crate::service::health_service;
use actix_web::web::{ServiceConfig, get};

pub fn register_endpoints(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health_service::health_service));
}
