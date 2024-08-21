pub(crate) trait SingleEntityModel<'a> where Self: Sized {
    //! Contains methods to perform basic DML operation over some table in DB.<br>
    //! There are some special methods:
    //! - `upsert()` **inserts** 1 row if it *doesn't exist* or **updates existing** row, in PostgreSQL dialect it is semantically equal to `INSERT ... ON CONFLICT UPDATE`;
    //! - `insert_or_skip()` **inserts** 1 row if it *doesn't exist* or **do nothing**, in PostgreSQL dialect it is semantically equal to `INSERT ... ON CONFLICT DO NOTHING`;
    
    type Session;
    type Id;
    type Error;
    type Create;

    async fn insert(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error>;
    async fn upsert(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error>;
    async fn insert_or_skip(s: &mut Self::Session, data: Self::Create) -> Result<Self, Self::Error>;
    async fn update(&self, s: &mut Self::Session) -> Result<Self, Self::Error>;
    async fn select(s: &mut Self::Session, id: Self::Id) -> Result<Self, Self::Error>;
    async fn delete(s: &mut Self::Session, id: Self::Id) -> Result<Self::Id, Self::Error>;
}

// trait BulkModel: SingleEntityModel {
//     fn select_many(s: Self::Session);
//     fn insert_many(s: Self::Session);
//     fn upsert(s: Self::Session);
//     fn insert_or_skip(s: Self::Session);
//     fn update(s: Self::Session);
//     fn delete_by_id(s: Self::Session);
//     fn delete_by_ids(s: Self::Session);
// }

pub mod arch;