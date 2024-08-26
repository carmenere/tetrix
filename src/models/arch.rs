use crate::db::{pg::Session, pgerr::DbError};
use crate::models::{Model, SelectRow, InsertRow, UpdateRow, DeleteRow, NextId};

#[derive(Debug, Clone)]
pub struct ArchRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ArchRowOptId {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

pub struct Arch;

impl Model for Arch {
    const name: &'static str = "arches";
    type Id = i64;
}

impl<'a> SelectRow<'a> for Arch {
    type Id = <Arch as Model>::Id;
    type Error = DbError;
    type Row = ArchRow;
    type Session = Session<'a>;

    async fn select(s: &mut Self::Session, id: Self::Id) -> Result<Self::Row, Self::Error> {
        Ok(sqlx::query_as!(
            Self::Row,
            r#"
                SELECT id, name, description FROM arches WHERE id = $1
            "#,
            id
        )
        .fetch_one(&mut **s)
        .await?)
    }
}

impl<'a> InsertRow<'a> for Arch {
    type Id = <Arch as Model>::Id;
    type Error = DbError;
    type Row = ArchRow;
    type Data = ArchRowOptId;
    type Session = Session<'a>;

    async fn insert(s: &mut Self::Session, data: Self::Data) -> Result<Self::Row, Self::Error> {
        match data.id {
            Some(id) => {
                Ok(sqlx::query_as!(
                        ArchRow,
                        r#"
                            INSERT INTO arches (id, name, description)
                            VALUES ($1, $2, $3)
                            RETURNING id, name, description
                        "#,
                        id,
                        data.name,
                        data.description
                    ).fetch_one(&mut **s).await?)
            },
            None => {
                Ok(sqlx::query_as!(
                        ArchRow,
                        r#"
                            INSERT INTO arches (name, description)
                            VALUES ($1, $2)
                            RETURNING id, name, description
                        "#,
                        data.name,
                        data.description
                    ).fetch_one(&mut **s).await?)
            },
        }
    }
}

impl<'a> UpdateRow<'a> for Arch {
    type Id = <Arch as Model>::Id;
    type Error = DbError;
    type Row = ArchRow;
    type Session = Session<'a>;

    async fn update(s: &mut Self::Session, data: Self::Row) -> Result<Self::Row, Self::Error> {
        Ok(sqlx::query_as!(
            ArchRow,
            r#"
                UPDATE arches SET name = $2, description = $3
                WHERE id = $1
                RETURNING id, name, description
            "#,
            data.id,
            data.name,
            data.description
            )
            .fetch_one(&mut **s)
            .await?)
    }
}


impl<'a> DeleteRow<'a> for Arch {
    type Id = <Arch as Model>::Id;
    type Error = DbError;
    type Session = Session<'a>;

    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(
                r#"DELETE FROM arches WHERE id = $1 RETURNING id AS "id: _" "#,
                id
        ).fetch_optional(&mut **s).await?.ok_or(DbError::NotFound)
    }
}

impl<'a> NextId<'a> for Arch {
    type Id = <Arch as Model>::Id;
    type Error = DbError;
    type Session = Session<'a>;
    
    async fn next_id(s: &mut Self::Session) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(r#"SELECT nextval('arches_id_seq'::regclass)"#)
        .fetch_one(&mut **s)
        .await?
        .ok_or(DbError::NotFound)
    }
}