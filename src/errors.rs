use actix_web::ResponseError;
use actix_web::web::HttpResponse;
use actix_web::http::StatusCode;
use serde::Serialize;
use serde::export::Formatter;

/// Representation of all possible Tallii error codes
#[derive(Serialize)]
pub enum TalliiError {
    DatabaseError,
}

/// Debub trait for the TalliiError
impl std::fmt::Debug for TalliiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TalliiError::DatabaseError => write!(f, "DatabaseError")
        }
    }
}

/// Display trait for a TalliiError
impl std::fmt::Display for TalliiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Represents a Tallii response when there is an error
#[derive(Debug, Serialize)]
pub struct TalliiErrorResponse {
    message: String,
    code: String
}

/// Display trait for TalliiErrorResponse
impl std::fmt::Display for TalliiErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Converts a reference to a TalliiError into a TalliiErrorResponse
impl From<&TalliiError> for TalliiErrorResponse {
    fn from(error: &TalliiError) -> TalliiErrorResponse {
        match error {
            TalliiError::DatabaseError => TalliiErrorResponse {
                message: "An unexpected error occurred in the database".to_string(),
                code: "DATABASE_ERROR".to_string()
            }
        }
    }
}

/// Converts a sqlx error into a TalliiError
impl From<sqlx::error::Error> for TalliiError {
    fn from(_error: sqlx::error::Error) -> TalliiError {
        TalliiError::DatabaseError
    }
}

/// ResponseError trait for the TalliiError
impl ResponseError for TalliiError {
    fn status_code(&self) -> StatusCode {
        match self {
            TalliiError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(TalliiErrorResponse::from(self))
    } 
}