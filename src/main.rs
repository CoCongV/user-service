#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{post, middleware, web, App, HttpServer, HttpResponse, Error};

mod db;
mod interface;
mod models;


#[post("/api/v1/generate_auth_token")]
async fn index(pool: web::Data<db::Pool>, info: web::Json<interface::Info>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || models::user::login(info, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        if let Some(user) = user {
            Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::NotFound()
                .body(format!("No user found"));
            Ok(res)
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_db_pool("postgres://cong:password@localhost:5432/userservice");
    // HttpServer::new(|| App::new().service(index))
    //     .bind("127.0.0.1:8080")?
    //     .run()
    //     .await
    HttpServer::new(move || {
        App::new().data(pool.clone())
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}