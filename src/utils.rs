use crate::errors::ServiceError;
use argon2::{self, Config};

use crate::config;
use crate::db;

pub fn hash_password(password: &str, salt: &[u8]) -> Result<String, ServiceError> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).map_err(|err| {
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

#[derive(Clone)]
pub struct AppState {
    pub conf: config::Config,
    pub pool: db::Pool,
}

pub fn generate_app_state() -> AppState {
    let conf = config::read_config();
    let pool = db::create_db_pool(&conf.db_url);
    AppState {
        conf,
        pool,
    }
}