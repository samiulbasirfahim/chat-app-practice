use actix_web::FromRequest;
use actix_web::HttpResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::error::InternalError;
use bcrypt::BcryptError;
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::encode;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

use crate::get_config;

use super::error_handler::build_error;

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

pub fn compare_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    let result = bcrypt::verify(password, hash)?;
    return Ok(result);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: i32,
    token_type: TokenType,
    exp: usize,
}

pub fn generate_token(
    user_id: i32,
    token_type: TokenType,
    expiry: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = get_config().jwt_secret.clone();
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
    let expiry = Utc::now()
        .checked_add_signed(Duration::minutes(expiry))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        token_type,
        exp: expiry,
    };

    Ok(encode(&Header::default(), &claims, &encoding_key)?)
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = get_config().jwt_secret.clone();
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = jsonwebtoken::Validation::default();

    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn tokens_generator(user_id: i32) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    let access_token = generate_token(user_id, TokenType::Access, 60)?;
    let refresh_token = generate_token(user_id, TokenType::Refresh, 60 * 24 * 7)?;
    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                let res =
                    HttpResponse::Unauthorized().json(build_error("Missing Authorization header"));
                return ready(Err(InternalError::from_response("Auth Error", res).into()));
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                let res = HttpResponse::Unauthorized().json(build_error("Invalid header format"));
                return ready(Err(InternalError::from_response("Auth Error", res).into()));
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Header must start with Bearer")));
        }

        let token = auth_str.trim_start_matches("Bearer ");
        let user_id = match decode_token(token) {
            Ok(payload) => {
                if payload.token_type == TokenType::Refresh {
                    let res = HttpResponse::Unauthorized().json(build_error("Wrong token type"));
                    return ready(Err(InternalError::from_response("Auth Error", res).into()));
                }
                payload.sub
            }
            Err(_) => {
                let res = HttpResponse::Unauthorized().json(build_error("Expired/Invalid token"));
                return ready(Err(InternalError::from_response("Auth Error", res).into()));
            }
        };

        ready(Ok(AuthUser { user_id }))
    }
}
