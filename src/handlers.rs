use crate::db;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::{PgPool, pool};

pub async fn root() -> &'static str {
    "Anime Recommend API"
}

// 一旦結果をターミナルに出してます
pub async fn list(State(pool): State<PgPool>) {
    match db::get_list(&pool).await {
        Ok(works) => println!("{:?}", works),
        Err(_) => println!("作品一覧を取得できませんでした"),
    }
}
