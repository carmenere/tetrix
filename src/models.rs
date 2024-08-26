use sqlx::postgres::PgPool;

enum Upsert {
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT UPDATE`
    Update,
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT DO NOTHING`
    DoNothing,
}

pub mod arch;

pub(crate) trait Model {
    const name: &'static str;
    type Id;
}

pub(crate) trait SelectRow<'a>
where
    Self: Sized,
{
    type Id;
    type Error;
    type Row;
    type Session;

    async fn select(s: &mut Self::Session, id: Self::Id) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait InsertRow<'a>
where
    Self: Sized,
{
    type Id;
    type Error;
    type Row;
    type Session;
    type Data;

    async fn insert(s: &mut Self::Session, data: Self::Data) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait UpdateRow<'a>
where
    Self: Sized,
{
    type Id;
    type Error;
    type Row;
    type Session;

    async fn update(s: &mut Self::Session, data: Self::Row) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait DeleteRow<'a>
where
    Self: Sized,
{
    type Id;
    type Error;
    type Session;

    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error>;
}

pub(crate) trait NextId<'a>
where
    Self: Sized,
{
    type Id;
    type Error;
    type Session;

    async fn next_id(s: &mut Self::Session) -> Result<Self::Id, Self::Error>;
}