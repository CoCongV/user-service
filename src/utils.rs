use crate::errors::ServiceError;
use argon2::{self, Config};

use crate::SALT;

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded(&hash, password.as_bytes()).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })
}