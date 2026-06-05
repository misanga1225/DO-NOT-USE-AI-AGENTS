use crate::db;
use crate::error::AppError;
use crate::models::{MediaType, Status, Work};
// use anyhow::Ok;
use axum::{extract::Json, extract::Query, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::option::Option;

#[derive(Deserialize)]
pub struct ListQuery {
    user_id: i32,
    status: Option<String>,
}

// JSONレスポンス共通の構造体（メッセージ1つを返す用）
#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn root() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Anime Recommend API".to_string(),
    })
}

// 作品のタイトル一覧を表示
pub async fn list(
    State(pool): State<PgPool>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<String>>, AppError> {
    let titles = db::get_list(&pool, query.user_id, query.status.as_deref()).await?;
    Ok(Json(titles))
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
) -> Result<Json<MessageResponse>, AppError> {
    match query.strategy {
        Strategy::Random => recommend_random(&pool, query.user_id).await,
        Strategy::Ai => recommend_ai(&pool, query.user_id).await,
    }
}

// ランダムに選ばれたおすすめ作品の表示
async fn recommend_random(pool: &PgPool, user_id: i32) -> Result<Json<MessageResponse>, AppError> {
    match db::picked_random(pool, user_id).await? {
        Some(title) => Ok(Json(MessageResponse { message: title })),
        None => Ok(Json(MessageResponse {
            message: "おすすめできる作品がありません".to_string(),
        })),
    }
}

// AIが作品をレコメンドする機能（準備中）
async fn recommend_ai(pool: &PgPool, user_id: i32) -> Result<Json<MessageResponse>, AppError> {
    // 作品一覧取得
    let _work_list = db::get_list(pool, user_id, None).await?;

    // APIを呼ぶ
    Ok(Json(MessageResponse {
        message: "準備中".to_string(),
    }))
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
) -> Result<Json<MessageResponse>, AppError> {
    let work = Work {
        title: body.title,
        author: body.author,
        description: body.description,
        episodes: body.episodes,
        media_type: body.media_type,
        genres: body.genres,
    };
    // DB登録処理呼び出し
    db::insert_work(&pool, &work, body.user_id, &body.status).await?;
    Ok(Json(MessageResponse {
        message: "登録しました".to_string(),
    }))
}
