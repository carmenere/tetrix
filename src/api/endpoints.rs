use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub enum ApiResponse<T>
where
    T: Serialize,
{
    OK,
    Json(T),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Json(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

pub mod arch;
pub mod version;
