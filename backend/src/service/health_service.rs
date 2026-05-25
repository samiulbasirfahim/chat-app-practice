use actix_web::Responder;

pub async fn health_service() -> impl Responder {
    "OK"
}
