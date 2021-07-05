use std::sync::Arc;

use actix_web::{post, get, web, Error, HttpResponse};

use crate::db;
use crate::interface;
use crate::models;

// #[get("/api/v1/verify_auth_token")]
// pub async fn verify_password() -> Result<HttpResponse, Error> {

// }

#[get("/api/v1/token")]
pub async fn generate_auth_token(
    pool: web::Data<db::Pool>,
    info: web::Json<interface::Info>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let info = Arc::new(info);

    let info_clone = Arc::clone(&info);
    let user = web::block(move || models::user::query_user(&info_clone, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        if user.verify_password(&info.password) {
            Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::Unauthorized().body(format!("password Error"));
            Ok(res)
        }
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found"));
        Ok(res)
    }
}
