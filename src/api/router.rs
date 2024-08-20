use axum::{routing::{get, post}, Router};
use super::state::ApiState;
use super::endpoints::version::build_version;
use super::endpoints::arch::get_arch;

pub fn router(state: ApiState) -> Router {
    Router::new()
    .route("/version", get(build_version))
    .route("/arches/:id", get(get_arch))
    .with_state(state)
}