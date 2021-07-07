use actix_web::{get, web, Error, HttpResponse};

use crate::errors;
use crate::interface;
use crate::models;
use crate::utils::AppState;

// #[get("/api/v1/token")]
// pub async fn verify_token() -> Result<HttpResponse, Error> {

// }

#[get("/api/v1/token")]
pub async fn generate_auth_token(
    app_state: web::Data<AppState>,
    info: web::Json<interface::Info>,
) -> Result<HttpResponse, Error> {
    let conn = app_state.pool.get().expect("couldn't get db connection from pool");
    let token = web::block(move || {
        let info = info.into_inner();
        let user = models::user::query_user(&info, &conn)?;
        if user.verify_password(&info.password) {
            Ok(user.generate_auth_token(&app_state.conf.secret_key, &app_state.conf.expires_at))
        } else {
            Err(errors::ServiceError::Unauthorized)
        }
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::Unauthorized().finish()
    })?;

    Ok(HttpResponse::Ok().json(interface::Token{
        token
    }))
}