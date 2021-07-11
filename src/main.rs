#[macro_use]
extern crate diesel;
extern crate clap;
extern crate dotenv;

use std::env;

use clap::SubCommand;
use diesel::PgConnection;
use once_cell::sync::Lazy;

mod api;
mod config;
mod db;
mod errors;
mod handles;
mod interface;
mod models;
mod utils;

use crate::config::Config;

static CONF: Lazy<Config> = Lazy::new(|| config::read_config());
static SALT: Lazy<&'static [u8]> = Lazy::new(|| CONF.salt.as_bytes());

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

#[tokio::main]
async fn runserver() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }

    let pool = db::create_db_pool(&CONF.db_url);
    let api = api::api_filters(pool);
    warp::serve(api).run(([0, 0, 0, 0], 8001)).await;
}

fn main() {
    let matches = clap::App::new("User Service")
        .subcommand(SubCommand::with_name("runserver").about("run server"))
        .subcommand(SubCommand::with_name("init").about("init database and create admin uesr"))
        .get_matches();
    if let Some(_) = matches.subcommand_matches("runserver") {
        runserver()
    }
    if let Some(_) = matches.subcommand_matches("init") {
        let pool = db::create_db_pool(&CONF.db_url);
        let conn = pool.get().expect("get diesel connection fail!");
        if let Ok(_) = init(&conn) {
            println!("init success!")
        }
    }
}
