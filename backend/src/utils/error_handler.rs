use actix_web::http::StatusCode;
use actix_web::web::JsonConfig;
use actix_web::{HttpResponse, error};
use actix_web_validator::{Error as ValidatorError, JsonConfig as ValidateJsonConfig};
use validator::ValidationError;

pub fn generate_error_handlers() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        let (status_code, error_message) = match &err {
            error::JsonPayloadError::Overflow { limit } => (
                StatusCode::BAD_REQUEST,
                format!("Exceeded payload limit: {}", limit),
            ),
            error::JsonPayloadError::Deserialize(serde_err) => {
                (StatusCode::BAD_REQUEST, serde_err.to_string())
            }
            _ => (StatusCode::BAD_REQUEST, "Something Went Wrong".to_string()),
        };
        let response = HttpResponse::build(status_code).json(serde_json::json!({
            "status": "error",
            "message": error_message
        }));

        actix_web::error::InternalError::from_response(err, response).into()
    })
}

pub fn generate_validation_handler() -> ValidateJsonConfig {
    ValidateJsonConfig::default().error_handler(|err, _| match &err {
        ValidatorError::Validate(validation_errors) => {
            let response = HttpResponse::UnprocessableEntity().json(validation_errors);
            error::InternalError::from_response(err, response).into()
        }
        _ => {
            let response = HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Invalid JSON payload"}));
            error::InternalError::from_response(err, response).into()
        }
    })
}
