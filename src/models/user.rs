extern crate bcrypt;

use actix_web::error::{Error, ErrorBadRequest, ErrorUnauthorized};
// use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors;
use crate::interface;
use crate::models::schema::users;
use crate::utils::{hash_password, verify};

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone, Debug)]
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
    pub fn new(
        name: String,
        email: String,
        avatar: String,
        verify: bool,
        password: String,
        role: i32,
    ) -> Result<NewUser, errors::ServiceError> {
        let password_hash = hash_password(&password)?;
        Ok(NewUser {
            name: name,
            email: email,
            avatar: avatar,
            verify: verify,
            password_hash: password_hash,
            role: role,
        })
    }
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
    pub fn generate_auth_token<'a>(&self, secret_key: &str, expires_at: &usize) -> String {
        let claims = Claims {
            uid: self.id,
            exp: *expires_at,
        };
        if let Ok(token) = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        ) {
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

// pub fn hash_password(password: String) -> String {
//     if let Ok(password_hash) = hash(password, DEFAULT_COST) {
//         password_hash
//     } else {
//         panic!("Generate Password Fail")
//     }
// }

pub fn verify_auth_token<'a>(
    secret: String,
    conn: &PgConnection,
    token: String,
) -> Result<Option<User>, Error> {
    use crate::models::schema::users::dsl::*;

    if let Ok(token_data) = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        if let Ok(user) = users
            .filter(id.eq(token_data.claims.uid))
            .first::<User>(conn)
            .optional()
        {
            Ok(user)
        } else {
            Err(ErrorUnauthorized("Unauthorized"))
        }
    } else {
        Err(ErrorBadRequest("token valid"))
    }
}

pub fn query_user(
    userinfo: &interface::Info,
    conn: &PgConnection,
) -> Result<User, errors::ServiceError> {
    use crate::models::schema::users::dsl::*;

    let mut items = users
        .filter(name.eq(&userinfo.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        Ok(user)
    } else {
        Err(errors::ServiceError::NotFound("User is not exists".into()))
    }
}

pub fn insert_new_user(
    username: String,
    useremail: String,
    password: String,
    useravatar: String,
    conn: &PgConnection,
) -> Result<User, diesel::result::Error> {
    let user = NewUser::new(username, useremail, useravatar, false, password, 1).unwrap();
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?;
    Ok(user)
}
