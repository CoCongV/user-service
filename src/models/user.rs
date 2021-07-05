extern crate bcrypt;

use actix_web::web;
use actix_web::error::{Error, InternalError, ErrorUnauthorized, ErrorBadRequest};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::interface;
use crate::models::schema::users;

#[derive(Queryable, Insertable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub verify: bool,
    pub password_hash: String,
    pub role: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: i32,
    exp: usize,
    
    
}

impl User {
    pub fn verify_password(&self, password: &String) -> bool {
        if let Ok(valid) = verify(&self.password_hash, password) {
            valid
        } else {
            false
        }
    }
    
    pub fn generate_auth_token<'a>(&self, secret_key: String, expires_at: usize) -> String {
        let claims = Claims {
            uid: self.id,
            exp: expires_at,
        };
        if let Ok(token) = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())) {
            token
        } else {
            panic!("generate auth token fail")
        }
    }

    pub fn set_password(&mut self, password: String) {
        if let Ok(password_hash) = hash(password, DEFAULT_COST) {
            self.password_hash = password_hash;
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::models::schema::users::dsl::*;
        diesel::insert_into(users).values(self).execute(conn)?;
        Ok(self.id)
    }

}

pub fn verify_auth_token<'a>(secret: String, conn: &PgConnection, token: String) -> Result<Option<User>, Error> {
    use crate::models::schema::users::dsl::*;

    if let Ok(token_data) = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
        if let Ok(user) = users.filter(id.eq(token_data.claims.uid)).first::<User>(conn).optional() {
            Ok(user)
        } else {
            Err(ErrorUnauthorized("Unauthorized"))
        }
    } else {
        Err(ErrorBadRequest("token valid"))
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

