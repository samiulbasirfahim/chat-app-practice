use bcrypt::{BcryptError, bcrypt};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

pub fn compare_password(password: String, hash: String) -> Result<bool, BcryptError> {
    let result = bcrypt::verify(password, hash.as_str())?;
    return Ok(result);
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub fn generate_token(userId: i32, expiry: usize) -> String {
    let encoding_key = EncodingKey::from_secret("my_super_secret".as_ref());
    "hell".to_owned()
}
