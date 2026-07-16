use actix_web::HttpResponse;
use serde_json::json;

pub fn success_generator(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": message
    }))
}
