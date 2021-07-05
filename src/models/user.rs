extern crate bcrypt;

use actix_web::web;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptResult};
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

impl User {
    pub fn verify_password(&self, password: &String) -> bool {
        if let Ok(valid) = verify(&self.password_hash, password) {
            valid
        } else {
            false
        }
    }
}

pub fn query_user(
    userinfo: &web::Json<interface::Info>,
    conn: &PgConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::models::schema::users::dsl::*;

    let user = users
        .filter(name.eq(&userinfo.username))
        .first(conn)
        .optional()?;

    Ok(user)
}

