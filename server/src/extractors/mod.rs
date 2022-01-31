use actix_web::{error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;

use derive_more::{Display, Error};
use diesel::result::DatabaseErrorKind;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "An internal error has occured!")]
    InternalServerError,
    #[display(fmt = "An item with the following data already exists in the database!")]
    UniqueViolation
}

impl From<DatabaseErrorKind> for UserError {
    fn from(db_error_kind: DatabaseErrorKind) -> Self {
        match db_error_kind {
            DatabaseErrorKind::UniqueViolation => Self::UniqueViolation,
            _ => Self::InternalServerError
        }
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::UniqueViolation => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}