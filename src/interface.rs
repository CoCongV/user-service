use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}