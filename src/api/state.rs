use crate::db::pg::Client as PgClient;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct ApiState {
    pub db: PgClient,
}

impl ApiState {
    pub async fn new() -> Self {
        Self {
            db: PgClient::new().await,
        }
    }
}

impl FromRef<ApiState> for PgClient {
    fn from_ref(app_state: &ApiState) -> PgClient {
        app_state.db.clone()
    }
}
