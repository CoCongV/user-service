#[macro_use]
extern crate diesel;
extern crate clap;
extern crate dotenv;
extern crate lazy_static;

use actix_web::{middleware, App, HttpServer};
use clap::SubCommand;
use diesel::PgConnection;
use once_cell::sync::Lazy;

mod api;
mod config;
mod db;
mod errors;
mod interface;
mod models;
mod utils;

use crate::config::Config;

static CONF: Lazy<Config> = Lazy::new(|| config::read_config());
static SALT: Lazy<&'static [u8]> = Lazy::new(|| CONF.salt.as_bytes());

#[actix_web::main]
async fn runserver() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let pool = db::create_db_pool(&CONF.db_url);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(api::token::generate_auth_token)
    })
    .bind(&CONF.addr)?
    .run()
    .await
}

fn init(conn: &PgConnection) -> std::io::Result<()> {
    let _conf = CONF.clone();
    models::user::insert_new_user(
        _conf.admin_user,
        _conf.admin_email,
        _conf.admin_password,
        _conf.default_avatar,
        conn,
    )
    .unwrap();
    Ok(())
}

fn main() {
    let matches = clap::App::new("User Service")
        .subcommand(SubCommand::with_name("runserver").about("run server"))
        .subcommand(SubCommand::with_name("init").about("init database and create admin uesr"))
        .get_matches();
    if let Some(_) = matches.subcommand_matches("runserver") {
        if let Ok(_) = runserver() {}
    }
    if let Some(_) = matches.subcommand_matches("init") {
        let pool = db::create_db_pool(&CONF.db_url);
        let conn = pool.get().expect("get diesel connection fail!");
        if let Ok(_) = init(&conn) {
            println!("init success!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt;
    use actix_web::{http, test, web};

    use crate::api::token::generate_auth_token;
    use crate::interface::Info;

    static POOL: Lazy<db::Pool> = Lazy::new(|| db::create_db_pool(&CONF.db_url));

    #[actix_rt::test]
    async fn test_get_token_ok() {
        let mut app =
            test::init_service(App::new().data(POOL.clone()).service(generate_auth_token)).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/token")
            .set_json(&Info {
                username: "admin".to_owned(),
                password: "password".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn get_token_not_ok() {
        let mut app =
            test::init_service(App::new().data(POOL.clone()).service(generate_auth_token)).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/token")
            .set_json(&Info {
                username: "admin".to_owned(),
                password: "admin".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 401);
    }
}
