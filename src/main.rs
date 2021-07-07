#[macro_use]
extern crate diesel;
extern crate clap;
extern crate dotenv;
extern crate lazy_static;

use actix_web::{middleware, App, HttpServer};
use clap::SubCommand;

mod api;
mod config;
mod db;
mod errors;
mod interface;
mod models;
mod utils;

#[actix_web::main]
async fn runserver() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let config = config::read_config();
    HttpServer::new(move || {
        App::new()
            .data(
                utils::generate_app_state()
            )
            .wrap(middleware::Logger::default())
            .service(api::token::generate_auth_token)
    })
    .bind(config.addr)?
    .run()
    .await
}

fn init() -> std::io::Result<()> {
    let conf = config::read_config();
    let conn = &db::create_db_pool(&conf.db_url).get().expect("get diesel connection fail!");
    models::user::insert_new_user(
        conf.admin_user,
        conf.admin_email,
        conf.admin_password,
        conf.default_avatar,
        conn,
        conf.salt.as_bytes(),
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
        if let Ok(_) = init() {
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

    #[actix_rt::test]
    async fn test_get_token_ok() {
        let mut app =
            test::init_service(App::new().data(utils::generate_app_state()).service(generate_auth_token)).await;
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
            test::init_service(App::new().data(utils::generate_app_state()).service(generate_auth_token)).await;
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
