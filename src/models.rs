use sqlx::postgres::PgPool;

enum Upsert {
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT UPDATE`
    Update,
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT DO NOTHING`
    DoNothing,
}

pub(crate) trait SingularModel<'a>
where
    Self: Sized,
{
    //! Contains methods to perform basic DML operation over some table in DB.<br>
    //! There are some special methods:
    //! - `upsert(mode: Upsert)` **inserts** 1 row if it *doesn't exist* or **updates existing** row if `mode=Upsert::Update` or **do nothing** if `mode=Upsert::DoNothing`

    type Session;
    type Id;
    type Data;
    type Error;

    async fn next_id(s: &mut Self::Session) -> Result<Self::Id, Self::Error>;
    async fn select(s: &mut Self::Session, id: Self::Id) -> Result<Self, Self::Error>;
    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error>;
    async fn insert(&self, s: &mut Self::Session) -> Result<Self, Self::Error>;
    async fn update(&self, s: &mut Self::Session) -> Result<Self, Self::Error>;
    async fn upsert(&self, s: &mut Self::Session, mode: Upsert) -> Result<Self, Self::Error>;
}

// trait BulkModel {
//     fn select_many(s: Self::Session);
//     fn insert_many(s: Self::Session);
//     fn upsert(s: Self::Session);
//     fn insert_or_skip(s: Self::Session);
//     fn update(s: Self::Session);
//     fn delete_by_id(s: Self::Session);
//     fn delete_by_ids(s: Self::Session);
// }

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