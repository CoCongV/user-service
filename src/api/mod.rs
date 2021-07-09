pub mod token;

use actix_web::{web, Scope};

pub fn generate_routes() -> Scope {
    web::scope("/api/v1").service(token::generate_auth_token).service(token::verify_password)
}
