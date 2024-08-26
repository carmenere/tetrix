use axum::extract::FromRef;
use crate::settings::Settings;

#[derive(Clone)]
pub struct ApiState {
    pub settings: Settings,
}

impl ApiState {
    pub async fn new() -> Self {
        Self {
            settings: Settings::new().await
        }
    }
}

impl FromRef<ApiState> for Settings {
    fn from_ref(app_state: &ApiState) -> Settings {
        app_state.settings.clone()
    }
}
