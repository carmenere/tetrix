use axum::{extract::{FromRef, Path, Query}, routing::{get, post}, Router};
use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};
use crate::{app_state::AppState};

use crate::handlers;

pub fn router(state: AppState) -> Router {
    Router::new()
    .route("/version", get(handlers::build_version))
    .with_state(state)
}