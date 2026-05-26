use crate::db;
use axum::{Json, extract::Query, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;
use std::option::Option;
use crate::models::{MediaType, Status};

#[derive(Deserialize)]
pub struct ListQuery {
    status: Option<String>,
}

pub async fn root() -> &'static str {
    "Anime Recommend API"
}

// 作品のタイトル一覧を表示
pub async fn list(
    State(pool): State<PgPool>,
    Query(query): Query<ListQuery>,
) -> Result<String, (StatusCode, String)> {
    match db::get_list(&pool, query.status.as_deref()).await {
        Ok(titles) => Ok(titles.join("\n")),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "作品一覧を取得できませんでした".to_string(),
        )),
    }
}

// ランダムに選ばれたおすすめ作品の表示
pub async fn random(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    match db::picked_random(&pool).await {
        Ok(Some(title)) => Ok(title),
        Ok(None) => Ok("おすすめできる作品がありません".to_string()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "おすすめ作品を取得できませんでした".to_string(),
        )),
    }
}

// worksエンドポイントの実装.途中．
// 試しに書いてみたやつだからどう扱っていただいても問題ないです！！！
#[derive(Deserialize)]
pub struct WorkRequest {
    pub title: String,
    pub author: String,
    pub description: String,
    pub episodes: Option<i32>,
    pub media_type: MediaType,
    pub genre: String,
    pub status: Status,
    pub added_at: String,
}