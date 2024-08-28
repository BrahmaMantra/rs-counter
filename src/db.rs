use std::env;

use sqlx::{ Pool, Sqlite, SqlitePool};
use time::PrimitiveDateTime;

pub async fn establish_connection()->Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("cannot fin env::DATABASE_URL");
    let pool = SqlitePool::connect(&database_url).await.expect("cannot connect to database");
    pool
}

#[derive(sqlx::FromRow )]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_key:String,
    pub created_at:PrimitiveDateTime,
    pub updated_at:PrimitiveDateTime,
}