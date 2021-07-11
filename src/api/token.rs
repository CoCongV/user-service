// use actix_web::{get, web, Error, HttpResponse};

// use crate::db;
// use crate::errors;
// use crate::interface;
// use crate::models;
// use crate::CONF;

// #[get("/api/v1/verify_auth_token")]
// pub async fn verify_password(user: web::Data<models::user::User>) -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().json(interface::VerifyOk{
//         uid: user.id,
//     }))
// }

// #[get("/api/v1/token")]
// pub async fn generate_auth_token(
//     pool: web::Data<db::Pool>,
//     info: web::Json<interface::Info>,
// ) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let token = web::block(move || {
//         let info = info.into_inner();
//         let user = models::user::query_user(&info, &conn)?;
//         if user.verify_password(&info.password) {
//             Ok(user.generate_auth_token(&CONF.secret_key, &CONF.expires_at))
//         } else {
//             Err(errors::ServiceError::Unauthorized)
//         }
//     })
//     .await
//     .map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::Unauthorized().finish()
//     })?;

//     Ok(HttpResponse::Ok().json(interface::Token{
//         token
//     }))
// }

use warp;
use warp::Filter;

use crate::db::Pool;
use crate::handles;
use crate::handles::utils::{with_json_body, with_pool};
use crate::interface::Info;

pub fn ping() ->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("verify")
        .and(warp::get())
        .and_then(handles::token::verify_token)
}

pub fn generate_auth_token(
    pool: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("token")
        .and(warp::get())
        .and(with_pool(pool))
        .and(with_json_body::<Info>())
        .and_then(handles::token::get_token)
}
