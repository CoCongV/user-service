use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}