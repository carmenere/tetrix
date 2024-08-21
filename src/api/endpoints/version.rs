use crate::api::endpoints::ApiResponse;
use crate::r#const::BUILD_VERSION;
use axum::{routing::get, Router};
use crate::api::state::ApiState;

pub async fn build_version<'a>() -> ApiResponse<&'a str> {
    ApiResponse::Json(BUILD_VERSION)
}

pub fn router() -> Router<ApiState> {
    Router::new()
    .route("/version", get(build_version))
}