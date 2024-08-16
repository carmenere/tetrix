use axum::{self};
use tokio;

use tlib::{self, app_state::AppState, router::router};

#[tokio::main]
async fn main() {
    let state = AppState::new().await;
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
