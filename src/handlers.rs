use crate::db;
use axum::{Json, extract::Query, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;
use std::option::Option;

#[derive(Deserialize)]
pub struct ListQuery {
    status: Option<String>,
}

pub async fn root() -> &'static str {
    "Anime Recommend API"
}

// 文字列だけブラウザに表示
pub async fn list(
    State(pool): State<PgPool>,
    Query(query): Query<ListQuery>,
) -> Result<String, (StatusCode, String)> {
    match db::get_list(&pool, query.status.as_deref()).await {
        Ok(works) => Ok(works.join("\n")),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "作品リストを取得できませんでした".to_string(),
        )),
    }
}
