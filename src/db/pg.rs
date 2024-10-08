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
        let postgres = "postgres".to_string();
        Self {
            user: env::var("PG_USER").unwrap_or(postgres.clone()),
            password: env::var("PG_PASSWORD").unwrap_or(postgres.clone()),
            db: env::var("PG_DB").unwrap_or(postgres),
            port: env::var("PG_PORT").unwrap_or("5432".to_string()).parse::<u16>().unwrap(),
            host: env::var("PG_HOST").unwrap_or("localhost".to_string()),
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

    pub async fn connect<'a>(&self) -> Result<DbClient<'a>, DbError> {
        Ok(DbClient::new(&self.pool).await?)
    }
}

pub struct DbClient<'a> {
    pub session: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> DbClient<'a> {
    pub async fn new(pool: &PgPool) -> Result<Self, DbError> {
        Ok(Self {
            session: pool.begin().await?,
        })
    }
    pub async fn commit(self) -> Result<(), DbError> {
        Ok(self.session.commit().await?)
    }
}
