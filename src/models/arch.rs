use sqlx::{Postgres, Transaction};

#[derive(Debug, Clone)]
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
    pub async fn select_by_id(s: &mut Transaction<'_, Postgres>, id: i64) -> Result<Arch, sqlx::Error> {
        Ok(sqlx::query_as!(Arch, 
            r#"SELECT id, name, description from arches WHERE id = $1"#, 
            id
        ).fetch_one(&mut **s).await?)
    }
}