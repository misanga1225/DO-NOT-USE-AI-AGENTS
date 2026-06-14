use crate::ai;
use crate::auth;
use crate::db;
use crate::error::AppError;
use crate::models::{MediaType, Status, Work};
use axum::{extract::Json, extract::Query, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    // クエリのstatusはStatusへパースして検証
    let status = query
        .status
        .as_deref()
        .map(|s| s.parse::<Status>())
        .transpose()
        .map_err(|_| AppError::BadRequest("不正なstatusです".to_string()))?;
    let titles = db::get_list(&pool, query.user_id, status.as_ref()).await?;
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
    NewAi,
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
        Strategy::Ai => recommend_with_ai(&pool, query.user_id, ai::RecommendMode::FromList).await,
        Strategy::NewAi => recommend_with_ai(&pool, query.user_id, ai::RecommendMode::New).await,
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

// AIが作品をレコメンドする機能
async fn recommend_with_ai(
    pool: &PgPool,
    user_id: i32,
    mode: ai::RecommendMode
) -> Result<Json<MessageResponse>, AppError> {
    // 作品一覧取得
    let completed_work_list = db::get_list(pool, user_id, Some(&Status::Completed)).await?;
    let notstarted_work_list = db::get_list(pool, user_id, Some(&Status::NotStarted)).await?;

    // 関数呼び出し
    let response_messeage =ai::recommend(completed_work_list, notstarted_work_list, mode).await?;

    // APIを呼ぶ
    Ok(Json(MessageResponse {
        message: response_messeage,
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

// 新規ユーザ登録用
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// ユーザ新規登録（パスワードはハッシュ化して保存）
pub async fn register(
    State(pool): State<PgPool>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // 平文のパスワードをハッシュ化
    let password_hash = auth::hash_password(&body.password)?;

    db::register(&pool, &body.name, &body.email, &password_hash)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return AppError::Conflict(
                        "このメールアドレスは既に登録されています".to_string(),
                    );
                }
            }
            AppError::Database(e)
        })?;

    Ok(Json(MessageResponse {
        message: "ユーザを登録しました".to_string(),
    }))
}

// ログイン用
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// メールアドレスとパスワードを照合
// 成功/失敗を返すだけで，トークン発行などのセッション管理は別途実装
pub async fn login(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let invalid = || AppError::Unauthorized("メールアドレスまたはパスワードが違います".to_string());

    let user = db::find_user_by_email(&pool, &body.email)
        .await?
        .ok_or_else(invalid)?;

    let password_hash = user.password_hash.ok_or_else(invalid)?;

    if !auth::verify_password(&body.password, &password_hash) {
        return Err(invalid());
    }

    Ok(Json(MessageResponse {
        message: "ログインしました".to_string(),
    }))
}
