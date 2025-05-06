use sea_query::{Alias, IntoIden, SeaRc, TableRef};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::core_config;

pub mod crud_fns;
mod utils;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    PgPoolOptions::new()
        .max_connections(core_config().db_max_conn().to_owned())
        .connect(core_config().db_url())
        .await
}

pub trait DbEntityOld {
    type IdType: Into<sea_query::Value>;
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SeaRc::new(Alias::new(Self::TABLE)).into_iden())
    }
}

pub trait DbEntity {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SeaRc::new(Alias::new(Self::TABLE)).into_iden())
    }
}
