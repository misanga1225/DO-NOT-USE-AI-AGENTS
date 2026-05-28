use crate::db;
use crate::models::{MediaType, Status, Work};
// use anyhow::Ok;
use axum::{extract::Query, extract::State, extract::Json, http::StatusCode};
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

// worksエンドポイントの実装
#[derive(Deserialize)]
pub struct WorkRequest {
    pub title: String,
    pub author: String,
    pub description: String,
    pub episodes: Option<i32>,
    pub media_type: MediaType,
    pub genres: Vec<String>,
    pub status: Status,
}

// 受け取ったデータをDBに保存する処理
pub async fn create_work(
    State(pool): State<PgPool>,
    Json(body): Json<WorkRequest>,
) -> Result<String, (StatusCode, String)> {
    let work = Work {
        title: body.title,
        author: body.author,
        description: body.description,
        episodes: body.episodes,
        media_type: body.media_type,
        genres: body.genres,
        status: body.status,
    };
    // DB登録処理呼び出し
    match db::insert_work(&pool, &work).await {
        Ok(_) => Ok("登録しました".to_string()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "登録に失敗しました".to_string(),
        )),  
    }
}