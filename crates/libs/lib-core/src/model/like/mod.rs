use crate::db::crud_fns::{count, delete, select, select_many};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct LikeRepo {
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub like_type: i16, // 1 = like, -1 = dislike
}

impl DbEntity for LikeRepo {
    const TABLE: &'static str = "likes";
}

#[derive(Serialize)]
pub struct LikeForCreate {
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub like_type: i16,
}

#[derive(Serialize, Default)]
pub struct LikeForSelect {
    pub id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub like_type: Option<i16>,
}

#[derive(Serialize, Default)]
pub struct LikeForDelete {
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub user_id: Uuid,
}

impl LikeRepo {
    pub async fn create(db: &Db, like_fc: LikeForCreate) -> Result<LikeRepo> {
        match (like_fc.post_id, like_fc.comment_id) {
            (Some(post_id), None) => {
                let query = r#"
                    INSERT INTO likes (post_id, user_id, like_type)
                    VALUES ($1, $2, $3::SMALLINT)
                    ON CONFLICT (post_id, user_id) WHERE post_id IS NOT NULL
                    DO UPDATE SET like_type = EXCLUDED.like_type
                    RETURNING *
                "#;

                let like = sqlx::query_as(query)
                    .bind(post_id)
                    .bind(like_fc.user_id)
                    .bind(like_fc.like_type)
                    .fetch_one(db)
                    .await?;

                Ok(like)
            }
            (None, Some(comment_id)) => {
                let query = r#"
                    INSERT INTO likes (comment_id, user_id, like_type)
                    VALUES ($1, $2, $3::SMALLINT)
                    ON CONFLICT (comment_id, user_id) WHERE comment_id IS NOT NULL
                    DO UPDATE SET like_type = EXCLUDED.like_type
                    RETURNING *
                "#;

                let like = sqlx::query_as(query)
                    .bind(comment_id)
                    .bind(like_fc.user_id)
                    .bind(like_fc.like_type)
                    .fetch_one(db)
                    .await?;

                Ok(like)
            }
            _ => Err(crate::error::Error::InvalidInput(
                "Either post_id or comment_id must be provided".into(),
            )),
        }
    }

    pub async fn find(db: &Db, like_fs: LikeForSelect) -> Result<LikeRepo> {
        select::<Self, _>(db, like_fs).await
    }

    pub async fn find_many(db: &Db, like_fs: LikeForSelect) -> Result<Vec<LikeRepo>> {
        select_many::<Self, _>(db, like_fs).await
    }

    pub async fn count(db: &Db, like_fs: LikeForSelect) -> Result<usize> {
        count::<Self, _>(db, like_fs).await
    }

    pub async fn delete(db: &Db, like_fd: LikeForDelete) -> Result<()> {
        delete::<Self, _>(db, like_fd).await
    }

    pub async fn get_post_rating(db: &Db, post_id: &Uuid) -> Result<i64> {
        let query = format!(
            "SELECT COALESCE(SUM(like_type), 0) as rating FROM {} WHERE post_id = $1",
            Self::TABLE
        );

        let rating: i64 = sqlx::query_scalar(&query)
            .bind(post_id)
            .fetch_one(db)
            .await?;

        Ok(rating)
    }

    pub async fn get_comment_rating(db: &Db, comment_id: &Uuid) -> Result<i64> {
        let query = format!(
            "SELECT COALESCE(SUM(like_type), 0) as rating FROM {} WHERE comment_id = $1",
            Self::TABLE
        );

        let rating: i64 = sqlx::query_scalar(&query)
            .bind(comment_id)
            .fetch_one(db)
            .await?;

        Ok(rating)
    }
}
