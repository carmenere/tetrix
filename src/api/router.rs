use axum::Router;
use super::endpoints::{arch, version};
use crate::api::state::ApiState;
// use tower_http::trace::TraceLayer;

pub fn router(state: ApiState) -> Router {
    Router::new()
    .nest("/", arch::router())
    .nest("/", version::router())
    .with_state(state)
    // .layer(TraceLayer::new_for_http())
}