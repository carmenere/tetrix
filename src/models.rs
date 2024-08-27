pub(crate) enum Upsert {
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT UPDATE`
    Update,
    /// In PostgreSQL dialect this variant is semantically equal to `INSERT ... ON CONFLICT DO NOTHING`
    DoNothing,
}

pub mod arch;

pub(crate) trait Model<'a> {
    const name: &'static str;
    type Id;
    type Error;
    type Row;
    type Session;
}

pub(crate) trait SelectRow<'a>: Model<'a>
where
    Self: Sized,
{
    async fn select(&mut self, id: Self::Id) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait InsertRow<'a>: Model<'a>
where
    Self: Sized,
{
    type Data;

    async fn insert(&mut self, data: Self::Data) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait UpdateRow<'a>: Model<'a>
where
    Self: Sized,
{
    async fn update(&mut self, data: Self::Row) -> Result<Self::Row, Self::Error>;
}

pub(crate) trait DeleteRow<'a>: Model<'a>
where
    Self: Sized,
{
    async fn delete(&mut self, id: Self::Id) -> Result<Self::Id, Self::Error>;
}

pub(crate) trait NextId<'a>: Model<'a>
where
    Self: Sized,
{
    async fn next_id(&mut self) -> Result<Self::Id, Self::Error>;
}
