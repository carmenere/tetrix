use crate::db::pg::Client as PgClient;

#[derive(Clone)]
pub struct Settings {
    pub pgclient: PgClient,
}

impl Settings {
    pub async fn new() -> Self {
        Self {
            pgclient: PgClient::new().await,
        }
    }
}

// struct Conf<'a, T> 
// where
//     T: Settings
// {
//     pub conf: &'a T
// }

// pub trait Settings {}

// pub struct Config<'a, T>
// where
//     T: Settings
// {
//     pub config: Vec<Conf<'a, T>>
// }

// impl<'a, T> Config<'a, T>
// where
//     T: Settings
// {
//     fn register(&mut self, s: Conf<'a, T>)  -> ()
//     where
//         T: Settings
//     {
//         self.config.push(s)
//     }
// }