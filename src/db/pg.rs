use super::pgerr::DbError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub type Session<'a> = sqlx::Transaction<'a, sqlx::Postgres>;

#[derive(Debug, Clone)]
pub struct ConnUrl {
    user: String,
    password: String,
    db: String,
    port: u16,
    host: String,
}

impl ToString for ConnUrl {
    fn to_string(&self) -> String {
        String::from(format!(
            "postgres://{0}:{1}@{2}:{3}/{4}",
            self.user, self.password, self.host, self.port, self.db
        ))
    }
}

impl ConnUrl {
    pub fn new() -> Self {
        Self {
            user: env::var("PG_USER").expect("PG_USER is not set."),
            password: env::var("PG_PASSWORD").expect("PG_PASSWORD is not set."),
            db: env::var("PG_DB").expect("PG_DB is not set."),
            port: env::var("PG_PORT")
                .expect("PG_PORT is not set.")
                .parse::<u16>()
                .unwrap(),
            host: env::var("PG_HOST").expect("PG_HOST is not set."),
        }
    }
}

#[derive(Clone)]
pub struct Client {
    pub pool: PgPool,
    pub conn_url: ConnUrl, 
}

impl Client {
    pub async fn new() -> Self {
        let c = ConnUrl::new();
        dbg!(&c);
        Self {
            pool: PgPoolOptions::new()
                .max_connections(10)
                .connect(&c.to_string())
                .await
                .unwrap(),
            conn_url: c,
        }
    }

    pub async fn connect(&self) -> Result<Conn, DbError> {
        Ok(Conn::new(&self.pool).await?)
    }
}


pub struct Conn<'a> {
    pub session: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> Conn<'a> {
    pub async fn new(pool: &PgPool) -> Result<Self, DbError> {
        Ok(Self {
            session: pool.begin().await?,
        })
    }
    pub async fn commit(self) -> Result<(), DbError> {
        Ok(self.session.commit().await?)
    }
}
