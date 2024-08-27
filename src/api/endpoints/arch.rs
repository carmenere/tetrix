use crate::api::endpoints::ApiResponse;
use crate::api::errors::AppError;
use crate::api::state::ApiState;
use crate::db::pg::Client as PgClient;
use crate::db::pgerr::DbError;
use crate::models::arch as m;
use crate::models::arch::{ArchRow, ArchRowOptId};
use crate::models::{DeleteRow, InsertRow, SelectRow, UpdateRow};
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ArchNoId {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
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

impl From<ArchNoId> for ArchRowOptId {
    fn from(v: ArchNoId) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
        }
    }
}

pub async fn get_arch(
    Path(id): Path<i64>,
    State(pg): State<PgClient>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = pg.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let r: ArchRow = arch.select(id).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(r.into()))
}

pub async fn create_arch(
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let arch = arch.insert(data.into()).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(arch.into()))
}

pub async fn update_arch(
    Path(id): Path<i64>,
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    dbg!(&data);
    let data = ArchRow {
        id,
        name: data.name,
        description: data.description,
    };
    let arch = arch.update(data).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(arch.into()))
}

pub async fn delete_arch(
    Path(id): Path<i64>,
    State(app): State<ApiState>,
) -> Result<ApiResponse<Id>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let id = arch.delete(id).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(Id { id: id }))
}

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/arches", post(create_arch))
        .route("/arches/:id", get(get_arch))
        .route("/arches/:id", put(update_arch))
        .route("/arches/:id", delete(delete_arch))
}
