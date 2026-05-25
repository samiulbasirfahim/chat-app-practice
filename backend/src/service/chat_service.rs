use actix_web::Responder;

pub async fn chat_service() -> impl Responder {
    "Chat endpoint"
}
