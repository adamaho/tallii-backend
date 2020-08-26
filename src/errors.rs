use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
use actix_web::ResponseError;
use serde::export::Formatter;
use serde::Serialize;

/// Representation of all possible Tallii error codes
pub enum TalliiError {
    DatabaseError,
    InternalServerError,
    InvalidInviteCode,
    Unauthorized,
}

#[derive(Debug)]
pub struct Test;

/// Debub trait for the TalliiError
impl std::fmt::Debug for TalliiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Display trait for a TalliiError
impl std::fmt::Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Represents a Tallii response when there is an error
#[derive(Debug, Serialize)]
pub struct TalliiErrorResponse {
    message: String,
    code: String,
}


/// Converts a sqlx error into a TalliiError
impl From<sqlx::error::Error> for TalliiError {
    fn from(_error: sqlx::error::Error) -> TalliiError {
        TalliiError::DatabaseError
    }
}

/// Converts a jsonwebtoken error into a TalliiError
impl From<jsonwebtoken::errors::Error> for TalliiError {
    fn from(_error: jsonwebtoken::errors::Error) -> TalliiError {
        TalliiError::Unauthorized
    }
}

/// ResponseError trait for the TalliiError
impl ResponseError for Test {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}
