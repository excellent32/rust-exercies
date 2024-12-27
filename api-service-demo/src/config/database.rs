use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub async fn do_connect() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect("mysql:://root:root123321@127.0.0.1/go-chain-data").await;

    pool.unwrap()
}