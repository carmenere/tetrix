use crate::api::endpoints::ApiResponse;
use crate::api::errors::AppError;
use crate::api::state::ApiState;
use crate::db::pg::Client as PgClient;
use crate::models::arch as m;
use crate::models::arch::{ArchRow, ArchRowOptId};
use crate::models::{DeleteRow, InsertRow, SelectRow, UpdateRow};
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::api::id::{ResourceId, Rid};
use serde_json::json;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArchNoId {
    pub id: Option<Rid<ArchId>>,
    pub name: String,
    pub description: Option<String>,
}


#[derive(Debug, Clone)]
pub(crate) struct ArchId {
    id: i64
}

impl ArchId {
    fn new(id: i64) -> Self {
        Self {
            id
        }
    }
}

impl ResourceId for ArchId {
    const PREFIX: &'static str = "arch_id";
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn new(id: Self::Id) -> Self {
        Self {
            id
        }
    }
    
    fn parse(s: &str) -> Result<Self::Id, std::num::ParseIntError> {
        let p = format!("{}-", Self::PREFIX);
        let r = match s.strip_prefix(&p) {
            Some(s) => Self::Id::from_str_radix(s, 16)?,
            None => Self::Id::from_str_radix(s, 10)?,
        };
        dbg!(r);
        Ok(r)
    }

}

#[derive(Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Arch {
    pub id: Rid<ArchId>,
    pub name: String,
    pub description: Option<String>,
}

impl From<ArchRow> for Arch {
    fn from(v: ArchRow) -> Self {
        Self {
            id: Rid(ArchId::new(v.id)),
            name: v.name,
            description: v.description,
        }
    }
}

impl From<ArchNoId> for ArchRowOptId {
    fn from(v: ArchNoId) -> Self {
        Self {
            id: v.id.map(|id| {id.0.id()}),
            name: v.name,
            description: v.description,
        }
    }
}

pub(crate) async fn get_arch(
    Path(id): Path<Rid<ArchId>>,
    State(pg): State<PgClient>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = pg.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let r: ArchRow = arch.select(id.0.id()).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(r.into()))
}

pub(crate) async fn create_arch(
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let arch = arch.insert(data.into()).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(arch.into()))
}

pub(crate) async fn update_arch(
    Path(id): Path<Rid<ArchId>>,
    State(app): State<ApiState>,
    Json(data): Json<ArchNoId>,
) -> Result<ApiResponse<Arch>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    dbg!(&data);
    let data = ArchRow {
        id: id.0.id(),
        name: data.name,
        description: data.description,
    };
    let arch = arch.update(data).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(arch.into()))
}

pub(crate) async fn delete_arch(
    Path(id): Path<Rid<ArchId>>,
    State(app): State<ApiState>,
) -> Result<ApiResponse<Rid<ArchId>>, AppError> {
    let mut db = app.db.connect().await?;
    let mut arch = m::Arch::new(&mut db.session);
    let id = arch.delete(id.0.id()).await?;
    let _ = db.commit().await?;
    Ok(ApiResponse::Json(Rid::<ArchId>(ArchId { id: id })))
}

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/arches", post(create_arch))
        .route("/arches/:id", get(get_arch))
        .route("/arches/:id", put(update_arch))
        .route("/arches/:id", delete(delete_arch))
}
