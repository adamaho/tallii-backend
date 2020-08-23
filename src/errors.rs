use serde::Serialize;

/// Representation of all possible Tallii error codes
pub enum TalliiErrorCode {
    DatabaseError,
}

/// Represents a Tallii response when there is an error
#[derive(Debug, Serialize)]
pub struct TalliiErrorResponse {
    message: String,
    code: i32,
}

/// Converts a TalliiErrorCode into a TalliiErrorResponse
impl From<TalliiErrorCode> for TalliiErrorResponse {
    fn from(error: TalliiErrorCode) -> TalliiErrorResponse {
        match error {
            TalliiErrorCode::DatabaseError => TalliiErrorResponse {
                message: "An unexpected error ocurred in the database".to_string(),
                code: 500,
            },
        }
    }
}

/// Converts a sqlx error into a TalliiErrorCode
impl From<sqlx::error::Error> for TalliiErrorCode {
    fn from(_error: sqlx::error::Error) -> TalliiErrorCode {
        TalliiErrorCode::DatabaseError
    }
}
