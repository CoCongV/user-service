use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::interface;

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub verify: bool,
    pub password_hash: String,
    pub role: i32,
}

pub fn login(
    userinfo: web::Json<interface::Info>,
    conn: &PgConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::models::schema::users::dsl::*;

    let user = users
        .filter(name.eq(&name))
        .first(conn)
        .optional()?;

    Ok(user)
}
