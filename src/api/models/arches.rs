use crate::api::endpoints::arch::{Arch, ArchNoId};
use crate::models::SingleEntityModel;

impl<'a> SingleEntityModel<'a> for Arch {
    type Session = sqlx::Transaction<'a, sqlx::Postgres>;
    type Id = i64;
    type Error = sqlx::Error;
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
        todo!()
    }

    async fn select(s: &mut Self::Session, id: i64) -> Result<Self, Self::Error> {
        Ok(sqlx::query_as!(Arch, 
            r#"SELECT id, name, description from arches WHERE id = $1"#, 
            id
        ).fetch_one(&mut **s).await?)
    }

    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error> {
        todo!()
    }
}
