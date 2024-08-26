use crate::db::{pg::Session, pgerr::DbError};
use super::{SingularModel, Upsert};

#[derive(Debug, Clone)]
pub struct ArchRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl<'a> SingularModel<'a> for ArchRow {
    type Session = Session<'a>;
    type Id = i64;
    type Error = DbError;
    type Data = ArchRow;

    async fn insert(&self, s: &mut Self::Session) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(
            ArchRow,
            r#"
                INSERT INTO arches (name, description) 
                VALUES ($1, $2) 
                RETURNING id, name, description"#,
            self.name,
            self.description
        )
        .fetch_one(&mut **s)
        .await?)
    }

    async fn upsert(&self, s: &mut Self::Session, mode: Upsert) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn update(&self, s: &mut Self::Session) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(
            ArchRow,
            r#"
                UPDATE arches SET name = $2, description = $3
                WHERE id = $1 
                RETURNING id, name, description"#,
            self.id,
            self.name,
            self.description
        )
        .fetch_one(&mut **s)
        .await?)
    }

    async fn select(s: &mut Self::Session, id: i64) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(
            ArchRow,
            r#"SELECT id, name, description FROM arches WHERE id = $1"#,
            id
        )
        .fetch_one(&mut **s)
        .await?)
    }

    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(
            r#"DELETE FROM arches WHERE id = $1 RETURNING id AS "id: _""#,
            id
        )
        .fetch_optional(&mut **s)
        .await?
        .ok_or(DbError::NotFound)
    }

    async fn next_id(s: &mut Self::Session) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(r#"SELECT nextval('arches_id_seq'::regclass)"#)
            .fetch_one(&mut **s)
            .await?
            .ok_or(DbError::NotFound)
    }
}

// async fn next_ids(s: &mut Self::Session, n: u32) -> Result<Self::Id, Self::Error> {
//     sqlx::query_scalar!(
//         r#"SELECT nextval('arches_id_seq') AS "sid!: _" FROM generate_series(1, $1)"#,
//         n as i64
//     ).fetch_one(&mut **s).await?.ok_or(DbError::DB("".to_owned()))
// }
