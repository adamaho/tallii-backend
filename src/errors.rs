use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
use actix_web::ResponseError;
use serde::export::Formatter;
use serde::Serialize;

/// External Representation of an error
#[derive(Debug, Serialize)]
pub struct TalliiError {
    pub code: TalliiErrorCode,
    pub message: String
}

/// Display trait for a TalliiError
impl std::fmt::Display for TalliiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl TalliiError {
    pub const INTERNAL_SERVER_ERROR: TalliiErrorCode = TalliiErrorCode("INTERNAL_SERVER_ERROR");
    pub const INVALID_INVITE_CODE: TalliiErrorCode = TalliiErrorCode("INVALID_INVITE_CODE");
    pub const UNAUTHORIZED: TalliiErrorCode = TalliiErrorCode("UNAUTHORIZED");
}

/// Representation of the TalliiErrorCodes
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct TalliiErrorCode(&'static str);

impl TalliiErrorCode {

    /// Provides the default code and message for the error
    pub fn default(self) -> TalliiError {
        // match on the message
        let message = match self {
            TalliiError::INTERNAL_SERVER_ERROR => "Oops, something seems to have gone wrong on our end.",
            TalliiError::INVALID_INVITE_CODE => "The provided invite code is invalid.",
            TalliiError::UNAUTHORIZED => "User does not have the permissions to fulfill request.",
            _ => "Oops, something seems to have gone wrong on our end."
        };

        // return the error
        TalliiError {
            code: self,
            message: message.to_string()
        }
    }

    /// Provides the ability to override the message
    pub fn message(self, message: String) -> TalliiError {
        TalliiError {
            code: self,
            message
        }
    }
}

/// Converts a TalliiErrorCode into a TalliiError
impl From<TalliiErrorCode> for TalliiError {
    fn from(error: TalliiErrorCode) -> TalliiError {
        error.default()
    }
}

/// Converts a jsonwebtoken error into a TalliiError
impl From<jsonwebtoken::errors::Error> for TalliiError {
    fn from(_error: jsonwebtoken::errors::Error) -> TalliiError {
        TalliiError::INTERNAL_SERVER_ERROR.default()
    }
}

/// Converts a sqlx error into a TalliiError
impl From<sqlx::error::Error> for TalliiError {
    fn from(_error: sqlx::error::Error) -> TalliiError {
        TalliiError::INTERNAL_SERVER_ERROR.default()
    }
}

/// ResponseError trait for the TalliiError
impl ResponseError for TalliiError {
    fn status_code(&self) -> StatusCode {
        match self.code {
            TalliiError::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
            TalliiError::INVALID_INVITE_CODE => StatusCode::BAD_REQUEST,
            TalliiError::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
