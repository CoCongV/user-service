use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}


#[derive(Deserialize, Serialize)]
pub struct VerifyOk{
    pub uid: i32,
}