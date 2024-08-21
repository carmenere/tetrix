use axum::extract::{Path, State};
use serde::{Deserialize,Serialize};
use crate::api::state::ApiState;
use crate::api::endpoints::ApiResponse;
use crate::api::errors::AppError;
use crate::models::SingleEntityModel;
use axum::Json;

use axum::{routing::{get, post, put}, Router};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub id: i64,
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

pub async fn update_arch(Path(id): Path<i64>, State(app): State<ApiState>, Json(data): Json<ArchNoId>) -> Result<ApiResponse<Arch>, AppError> {
    let mut s: sqlx::Transaction<'_, sqlx::Postgres> = app.pool.begin().await.unwrap();
    dbg!(&data);
    let model = Arch {
        id: id,
        name: data.name,
        description: data.description
    };
    let p = model.update(&mut s).await?;
    let _ = s.commit().await?;
    Ok(ApiResponse::Json(p))
}

pub async fn delete_arch(Path(id): Path<i64>, State(app): State<ApiState>) -> Result<ApiResponse<Id>, AppError> {
    let mut s: sqlx::Transaction<'_, sqlx::Postgres> = app.pool.begin().await.unwrap();
    let p = Arch::delete(&mut s, id).await?;
    let _ = s.commit().await?;
    Ok(ApiResponse::Json(Id{id: id}))
}

pub fn router() -> Router<ApiState> {
    Router::new()
    .route("/arches", post(create_arch))
    .route("/arches/:id", get(get_arch).delete(delete_arch))
    .route("/arches/:id", put(update_arch))
}

