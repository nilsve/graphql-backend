use std::fmt::{Display, Formatter};
use actix_web::ResponseError;
use anyhow::Error;

use crate::prelude::DynamoRepositoryError;

impl ResponseError for DynamoRepositoryError {}

pub type ActixAnyhow<T> = Result<T, ActixAnyhowError>;

#[derive(Debug)]
pub struct ActixAnyhowError {
    err: anyhow::Error,
}

impl Display for ActixAnyhowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{}", self.err).fmt(f)
    }
}

impl ResponseError for ActixAnyhowError {}

impl From<anyhow::Error> for ActixAnyhowError {
    fn from(value: Error) -> Self {
        Self { err: value }
    }
}