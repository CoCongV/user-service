#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, App, HttpServer};


mod db;
mod interface;
mod models;
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_db_pool("postgres://cong:password@localhost:5432/userservice");
    HttpServer::new(move || {
        App::new().data(pool.clone())
        .wrap(middleware::Logger::default())
        .service(api::user::generate_auth_token)
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}