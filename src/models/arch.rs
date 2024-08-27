use crate::db::{pg::Session, pgerr::DbError};
use crate::models::{DeleteRow, InsertRow, Model, NextId, SelectRow, UpdateRow};

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

pub struct Arch<'a, 'b> {
    session: &'b mut Session<'a>,
}

impl<'a, 'b> Arch<'a, 'b>
where
    'a: 'b,
{
    pub fn new(session: &'b mut Session<'a>) -> Self {
        Self { session }
    }
}

impl<'a> Model<'a> for Arch<'a, '_> {
    const name: &'static str = "arches";
    type Id = i64;
    type Error = DbError;
    type Row = ArchRow;
    type Session = Session<'a>;
}

impl<'a> SelectRow<'a> for Arch<'a, '_> {
    async fn select(&mut self, id: Self::Id) -> Result<Self::Row, Self::Error> {
        Ok(sqlx::query_as!(
            Self::Row,
            r#"
                SELECT id, name, description FROM arches WHERE id = $1
            "#,
            id
        )
        .fetch_one(&mut **self.session)
        .await?)
    }
}

impl<'a> InsertRow<'a> for Arch<'a, '_> {
    type Data = ArchRowOptId;

    async fn insert(&mut self, data: Self::Data) -> Result<Self::Row, Self::Error> {
        match data.id {
            Some(id) => Ok(sqlx::query_as!(
                ArchRow,
                r#"
                            INSERT INTO arches (id, name, description)
                            VALUES ($1, $2, $3)
                            RETURNING id, name, description
                        "#,
                id,
                data.name,
                data.description
            )
            .fetch_one(&mut **self.session)
            .await?),
            None => Ok(sqlx::query_as!(
                ArchRow,
                r#"
                            INSERT INTO arches (name, description)
                            VALUES ($1, $2)
                            RETURNING id, name, description
                        "#,
                data.name,
                data.description
            )
            .fetch_one(&mut **self.session)
            .await?),
        }
    }
}

impl<'a> UpdateRow<'a> for Arch<'a, '_> {
    async fn update(&mut self, data: Self::Row) -> Result<Self::Row, Self::Error> {
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
        .fetch_one(&mut **self.session)
        .await?)
    }
}

impl<'a> DeleteRow<'a> for Arch<'a, '_> {
    async fn delete(&mut self, id: Self::Id) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(
            r#"DELETE FROM arches WHERE id = $1 RETURNING id AS "id: _" "#,
            id
        )
        .fetch_optional(&mut **self.session)
        .await?
        .ok_or(DbError::NotFound)
    }
}

impl<'a> NextId<'a> for Arch<'a, '_> {
    async fn next_id(&mut self) -> Result<Self::Id, Self::Error> {
        sqlx::query_scalar!(r#"SELECT nextval('arches_id_seq'::regclass)"#)
            .fetch_one(&mut **self.session)
            .await?
            .ok_or(DbError::NotFound)
    }
}
