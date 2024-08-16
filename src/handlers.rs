use axum::{extract::{FromRef, Path, Query}, routing::{get, post}, Router};
use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use crate::{app_state::AppState};
use crate::version;
use crate::settings::Setings;

#[derive(Serialize)]
pub enum ApiResponse<T>
where
    T: Serialize
{
    OK,
    Json(T),
}

impl FromRef<AppState> for Setings {
    fn from_ref(app_state: &AppState) -> Setings {
        app_state.settings.clone()
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Json(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}

// pub const BUILD_VERSION: &str = env!("BUILD_VERSION");

pub async fn build_version<'a>() -> ApiResponse<&'a str> {
    ApiResponse::Json(version::BUILD_VERSION)
}

