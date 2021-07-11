use warp::{http::StatusCode, Rejection};

use crate::db::Pool;
use crate::errors::{AppError, ErrorType};
use crate::handles::utils::respond;
use crate::interface::{Info, Token};
use crate::models;
use crate::CONF;

pub async fn get_token(pool: Pool, info: Info) -> Result<impl warp::Reply, Rejection> {
    let conn = pool.get().expect("get database connection fail");
    let user = models::user::query_user(&info, &conn)?;
    if !user.verify_password(&info.password) {
        let err = AppError::new("password errors", ErrorType::Unauthorized);
        return respond(Err(err), StatusCode::default());
    }
    let token = user.generate_auth_token(&CONF.secret_key, &CONF.expires_at);
    match token {
        Ok(token) => respond(Ok(Token { token }), StatusCode::OK),
        Err(err) => respond(Err(err), StatusCode::default())
    }
}

pub async fn verify_token() -> Result<impl warp::Reply, Rejection> {
    return Ok(StatusCode::BAD_REQUEST);
}