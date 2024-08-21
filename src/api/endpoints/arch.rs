use axum::extract::{Path, State};
use serde::{Deserialize,Serialize};
use crate::api::state::ApiState;
use crate::api::endpoints::ApiResponse;
use crate::api::errors::AppError;
use crate::models::SingleEntityModel;
use axum::Json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArchNoId {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Arch {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl Arch {
    pub fn new (id: i64, name: String, description: Option<String>) -> Self {
        Arch {
            id,
            name,
            description,
        }
    }
}

pub async fn get_arch(Path(id): Path<i64>, State(app): State<ApiState>) -> Result<ApiResponse<Arch>, AppError> {
    let mut s: sqlx::Transaction<'_, sqlx::Postgres> = app.pool.begin().await.unwrap();
    let p = Arch::select(&mut s, id).await?;
    let _ = s.commit().await?;
    Ok(ApiResponse::Json(p))
}

pub async fn create_arch(State(app): State<ApiState>, Json(data): Json<ArchNoId>) -> Result<ApiResponse<Arch>, AppError> {
    let mut s: sqlx::Transaction<'_, sqlx::Postgres> = app.pool.begin().await.unwrap();
    let p = Arch::insert(&mut s, data).await?;
    let _ = s.commit().await?;
    Ok(ApiResponse::Json(p))
}