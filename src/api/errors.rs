use crate::models::pgerr::DbError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DB(DbError),
    Serialize(String),
    Deserialize(String),
    UnprocessableInput(String),
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DB(e) => write!(f, "DB error"),
            AppError::Serialize(e) => write!(f, "Serialize error"),
            AppError::Deserialize(e) => write!(f, "Deserialize error"),
            AppError::UnprocessableInput(e) => write!(f, "Unprocessable input"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DB(e) => match e {
                DbError::NotFound => (StatusCode::NOT_FOUND).into_response(),
                DbError::Constraint(e) => {
                    let j = json!({
                        "code": e.code,
                        "violation_type": e.violation_type,
                        "constraint": e.constraint,
                        "message": e.message,
                    });
                    (StatusCode::BAD_REQUEST, j.to_string()).into_response()
                }
            },
            AppError::Serialize(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            AppError::Deserialize(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            AppError::UnprocessableInput(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, e).into_response()
            }
        }
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        AppError::DB(e)
    }
}
