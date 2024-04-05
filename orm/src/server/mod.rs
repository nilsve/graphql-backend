use actix_web::ResponseError;

use crate::prelude::DynamoRepositoryError;

impl ResponseError for DynamoRepositoryError {}
