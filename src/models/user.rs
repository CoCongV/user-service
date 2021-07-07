extern crate bcrypt;

use actix_web::web;
use actix_web::error::{Error, ErrorUnauthorized, ErrorBadRequest};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::interface;
use crate::models::schema::users;

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub verify: bool,
    pub password_hash: String,
    pub role: i32,
}

#[derive(Deserialize, Serialize, Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub verify: bool,
    pub password_hash: String,
    pub role: i32,
}

impl NewUser {
    pub fn new(name: String, email: String, avatar: String, verify: bool, password: String, role: i32) -> NewUser {
        NewUser {
            name: name,
            email: email,
            avatar: avatar,
            verify: verify,
            password_hash: hash_password(password),
            role: role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: i32,
    exp: usize,
}

impl User {
    pub fn verify_password(&self, password: &String) -> bool {
        if let Ok(valid) = verify(password, &self.password_hash) {
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

    pub fn insert(&self, conn: &PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::models::schema::users::dsl::*;
        diesel::insert_into(users).values(self).execute(conn)?;
        Ok(self.id)
    }

}

pub fn hash_password(password: String) -> String {
    if let Ok(password_hash) = hash(password, DEFAULT_COST) {
        password_hash
    } else {
        panic!("Generate Password Fail")
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

pub fn insert_new_user(
    username: String,
    useremail: String,
    password: String,
    useravatar: String,
    conn: &PgConnection
) -> Result<User, diesel::result::Error> {
    let user = NewUser::new(username, useremail, useravatar, false, password, 1);
    let user = diesel::insert_into(users::table).values(&user).get_result(conn)?;
    Ok(user)
}
