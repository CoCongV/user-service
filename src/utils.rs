use argon2::{self, Config};

use crate::errors::{AppError, ErrorType};

use crate::SALT;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        AppError::new(&err.to_string(), ErrorType::Internal)
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AppError> {
    argon2::verify_encoded(&hash, password.as_bytes()).map_err(|err| {
        AppError::new(&err.to_string(), ErrorType::Unauthorized)
    })
}
