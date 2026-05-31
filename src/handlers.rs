use crate::db;
use crate::models::{MediaType, Status, Work};
// use anyhow::Ok;
use axum::{extract::Json, extract::Query, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;
use std::option::Option;

#[derive(Deserialize)]
pub struct ListQuery {
    user_id: i32,
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
    match db::get_list(&pool, query.user_id, query.status.as_deref()).await {
        Ok(titles) => Ok(titles.join("\n")),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "作品一覧を取得できませんでした".to_string(),
        )),
    }
}

// レコメンドの選定方法
// クエリパラメータ分岐用
#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Strategy {
    #[default]
    Random,
    Ai,
}

#[derive(Deserialize)]
pub struct RecommendQuery {
    user_id: i32,
    #[serde(default)]
    strategy: Strategy,
}

// おすすめ作品の表示
// strategyでおすすめ法を振り分け
pub async fn recommendations(
    State(pool): State<PgPool>,
    Query(query): Query<RecommendQuery>,
) -> Result<String, (StatusCode, String)> {
    match query.strategy {
        Strategy::Random => recommend_random(&pool, query.user_id).await,
        Strategy::Ai => recommend_ai(&pool, query.user_id).await,
    }
}

// ランダムに選ばれたおすすめ作品の表示
async fn recommend_random(pool: &PgPool, user_id: i32) -> Result<String, (StatusCode, String)> {
    match db::picked_random(pool, user_id).await {
        Ok(Some(title)) => Ok(title),
        Ok(None) => Ok("おすすめできる作品がありません".to_string()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "おすすめ作品を取得できませんでした".to_string(),
        )),
    }
}

// AIが作品をレコメンドする機能（準備中）
async fn recommend_ai(_pool: &PgPool, _user_id: i32) -> Result<String, (StatusCode, String)> {
    // 作品一覧取得
    // APIを呼ぶ

    Ok("準備中".to_string())
}

// worksエンドポイントの実装
#[derive(Deserialize)]
pub struct WorkRequest {
    pub user_id: i32,
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
    };
    // DB登録処理呼び出し
    match db::insert_work(&pool, &work, body.user_id, &body.status).await {
        Ok(_) => Ok("登録しました".to_string()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "登録に失敗しました".to_string(),
        )),
    }
}
