use crate::api::endpoints::ApiResponse;
use crate::api::errors::AppError;
use crate::api::state::ApiState;
use crate::models::arch::ArchRow;
use crate::models::SingularModel;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

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

impl From<ArchRow> for Arch {
    fn from(v: ArchRow) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
        }
    }
}

pub async fn get_arch(
    Path(id): Path<i64>,
    State(app): State<ApiState>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut pg = app.settings.pgclient.connect().await?;
    let p = ArchRow::select(&mut pg.session, id).await?;
    let _ = pg.commit().await?;
    Ok(ApiResponse::Json(p.into()))
}

pub async fn create_arch(
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut pg = app.settings.pgclient.connect().await?;
    let model = ArchRow {
        id: ArchRow::next_id(&mut pg.session).await?,
        name: data.name,
        description: data.description,
    };
    let p = model.insert(&mut pg.session).await?;
    let _ = pg.commit().await?;
    Ok(ApiResponse::Json(p.into()))
}

pub async fn update_arch(
    Path(id): Path<i64>,
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut pg = app.settings.pgclient.connect().await?;
    dbg!(&data);
    let model = ArchRow {
        id,
        name: data.name,
        description: data.description,
    };
    let p = model.update(&mut pg.session).await?;
    let _ = pg.commit().await?;
    Ok(ApiResponse::Json(p.into()))
}

pub async fn delete_arch(
    Path(id): Path<i64>,
    State(app): State<ApiState>,
) -> Result<ApiResponse<Id>, AppError> {
    let mut pg = app.settings.pgclient.connect().await?;
    let p = ArchRow::delete(&mut pg.session, id).await?;
    let _ = pg.commit().await?;
    Ok(ApiResponse::Json(Id { id: id }))
}

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/arches", post(create_arch))
        .route("/arches/:id", get(get_arch))
        .route("/arches/:id", put(update_arch))
        .route("/arches/:id", delete(delete_arch))
}
