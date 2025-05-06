use sea_query::{Alias, Asterisk, Expr, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::utils::{prepare_sea_query_fields, struct_to_vec};
use crate::error::{Error, Result};

use super::{Db, DbEntity};

pub async fn create<T, Fc>(db: &Db, fc: Fc) -> Result<T>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fc: Serialize,
{
    let fc_vec = struct_to_vec(&fc);
    let (columns, sea_values) = prepare_sea_query_fields(fc_vec);

    let (sql, values) = Query::insert()
        .into_table(T::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(result)
}

pub async fn select_all<T>(db: &Db) -> Result<Vec<T>>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let (sql, values) = Query::select()
        .from(T::table_ref())
        .columns([Asterisk])
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn select<T, Fs>(db: &Db, fs: Fs) -> Result<T>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fs: Serialize + Sync,
{
    let fs_vec = struct_to_vec(&fs);
    let (columns, sea_values) = prepare_sea_query_fields(fs_vec);

    let mut query = Query::select();
    query.from(T::table_ref());
    query.columns([Asterisk]);
    for (column, value) in columns.iter().zip(sea_values) {
        query.and_where(Expr::col(column.to_owned()).eq(value.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound)?;

    Ok(result)
}

pub async fn select_many<T, Fs>(db: &Db, fs: Fs) -> Result<Vec<T>>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fs: Serialize + Sync,
{
    let fs_vec = struct_to_vec(&fs);
    let (columns, sea_values) = prepare_sea_query_fields(fs_vec);

    let mut query = Query::select();
    query.from(T::table_ref());
    query.columns([Asterisk]);
    for (column, value) in columns.iter().zip(sea_values) {
        query.and_where(Expr::col(column.to_owned()).eq(value.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn select_many_with_join<T, J, Fs>(
    db: &Db,
    fs: Fs,
    join_column_main: &str,
    join_column_other: &str,
) -> Result<Vec<T>>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    J: DbEntity,
    Fs: Serialize + Sync,
{
    let fs_vec = struct_to_vec(&fs);
    let (columns, sea_values) = prepare_sea_query_fields(fs_vec);

    let mut query = Query::select();
    query.from(T::table_ref()).columns([Asterisk]).join(
        JoinType::InnerJoin,
        J::table_ref(),
        Expr::col(Alias::new(format!("{}.{}", T::TABLE, join_column_main)))
            .equals(Alias::new(format!("{}.{}", T::TABLE, join_column_other))),
    );
    for (column, value) in columns.iter().zip(sea_values) {
        query.and_where(Expr::col(column.to_owned()).eq(value.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn update<T, Fu>(db: &Db, id: &Uuid, fu: Fu) -> Result<T>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fu: Serialize,
{
    let fs_vec = struct_to_vec(&fu);
    let (columns, sea_values) = prepare_sea_query_fields(fs_vec);

    let mut query = Query::update();
    query.table(T::table_ref());
    for (column, value) in columns.iter().zip(sea_values) {
        query.value(column.to_owned(), value);
    }
    query.and_where(Expr::col(Alias::new("id")).eq(*id));
    query.returning_all();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, T, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound)?;

    Ok(result)
}

pub async fn delete<T, Fd>(db: &Db, fd: Fd) -> Result<()>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fd: Serialize,
{
    let fd_vec = struct_to_vec(&fd);
    let (columns, sea_values) = prepare_sea_query_fields(fd_vec);

    let mut query = Query::delete();
    query.from_table(T::table_ref());
    for (column, value) in columns.iter().zip(sea_values) {
        query.and_where(Expr::col(column.to_owned()).eq(value.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(db).await?;

    if result.rows_affected() == 0 {
        return Err(Error::EntityNotFound);
    }

    Ok(())
}

pub async fn count<T, Fs>(db: &Db, fs: Fs) -> Result<usize>
where
    T: DbEntity + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    Fs: Serialize,
{
    let fs_vec = struct_to_vec(&fs);
    let (columns, sea_values) = prepare_sea_query_fields(fs_vec);

    let mut query = Query::select();
    query.from(T::table_ref());
    query.expr(Expr::col(Alias::new("id")).count());
    for (column, value) in columns.iter().zip(sea_values) {
        query.and_where(Expr::col(column.to_owned()).eq(value.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let count: (i64,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(count.0 as usize)
}

// pub async fn select_limit<T>(db: &Db) -> Result<Vec<T>> {
//     todo!()
// }
