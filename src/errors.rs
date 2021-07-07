use actix_web::error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", name)]
pub struct CustomError {
    pub name: &'static str,
}

impl error::ResponseError for CustomError {}