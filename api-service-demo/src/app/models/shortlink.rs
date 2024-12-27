use sqlx::{Error, MySql, Pool, FromRow};
use sql::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};

pub async fn create_shortlink(pool: &Pool<MySql>, url: &str) -> Result<MySqlQueryResult, Error>{
    sqlx::query(
        r#"
            insert into short_links (`url`)
            values(?)
        "#,
    ).bind(url)
    .execute(pool).await
}

pub async fn delete_shortlink(pool: &Pool<MySql>, id:u64) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
        delete from short_links
        where id = ?
        "#
    ).bind(id)
    .execute(pool).await
}

pub async fn get_shortlink(pool: &Pool<MySql>, id:i32) -> Result<Shortlink, Error> {
    sqlx::query_as::<_, ShortLink>(
        r#"
            select * from short_links
            where id = ?
        "#,
    ).bind(id)
    .fetch_one(pool).await
}