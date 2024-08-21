use crate::api::endpoints::arch::{Arch, ArchNoId};
use crate::models::SingleEntityModel;
use crate::api::errors::AppError;

impl<'a> SingleEntityModel<'a> for Arch {
    type Session = sqlx::Transaction<'a, sqlx::Postgres>;
    type Id = i64;
    type Error = AppError;
    type Create = ArchNoId;

    async fn insert(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(Arch, 
            r#"
                INSERT INTO arches (name, description) 
                VALUES ($1, $2) 
                RETURNING id, name, description"#, 
                data.name, data.description
        ).fetch_one(&mut **s).await?)
    }

    async fn upsert(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn insert_or_skip(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn update(&self, s: &mut Self::Session) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(Arch, 
            r#"
                UPDATE arches SET name = $2, description = $3
                WHERE id = $1 
                RETURNING id, name, description"#, 
                self.id, self.name, self.description
        ).fetch_one(&mut **s).await?)
    }

    async fn select(s: &mut Self::Session, id: i64) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(Arch, 
            r#"SELECT id, name, description FROM arches WHERE id = $1"#, 
            id
        ).fetch_one(&mut **s).await?)
    }

    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!( 
            r#"DELETE FROM arches WHERE id = $1 RETURNING id AS "id: _""#, 
            id
        ).fetch_optional(&mut **s).await?.ok_or(AppError::DB(format!("Missing entity: arch with id = {}.", id)))
    }
}
