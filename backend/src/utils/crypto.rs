use bcrypt::{BcryptError, bcrypt};

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

pub fn compare_password(password: String, hash: String) -> Result<bool, BcryptError> {
    let result = bcrypt::verify(password, hash.as_str())?;
    return Ok(result);
}
