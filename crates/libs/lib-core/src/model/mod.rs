use std::sync::Arc;

use crate::cache::{new_cache_pool, Cache};
use crate::db::{new_db_pool, Db};
use crate::error::Result;

pub mod chat;
pub mod chat_member;
pub mod chat_role;
pub mod comment;
pub mod community;
pub mod follow;
pub mod like;
pub mod message;
pub mod message_status;
pub mod post;
pub mod report;
pub mod role;
pub mod save;
pub mod token;
pub mod user;

#[derive(Debug, Clone)]
pub struct ModelManager {
    db: Arc<Db>,
    cache: Arc<Cache>,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = Arc::new(new_db_pool().await?);
        let cache = Arc::new(new_cache_pool().await?);
        Ok(Self { db, cache })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
