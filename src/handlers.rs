use crate::ai;
use crate::auth;
use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{MediaType, Status, Work};
use crate::state::AppState;
use axum::{extract::Json, extract::Path, extract::Query, extract::State};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct ListQuery {
    status: Option<String>,
}

// JSONレスポンス共通の構造体（メッセージ1つを返す用）
#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct RecommendResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
}

#[derive(Serialize)]
pub struct AiUsageResponse {
    pub remaining: i32,
    pub limit: i32,
}

pub async fn root() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Anime Recommend API".to_string(),
    })
}

// 作品のタイトル一覧を表示
pub async fn list(
    State(pool): State<PgPool>,
    user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<String>>, AppError> {
    let status = query
        .status
        .as_deref()
        .map(|s| s.parse::<Status>())
        .transpose()
        .map_err(|_| AppError::BadRequest("不正なstatusです".to_string()))?;
    let titles = db::get_list(&pool, user.user_id, status.as_ref()).await?;
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
    #[serde(default)]
    strategy: Strategy,
}

// おすすめ作品の表示
// strategyでおすすめ法を振り分け
pub async fn recommendations(
    State(pool): State<PgPool>,
    user: AuthUser,
    Query(query): Query<RecommendQuery>,
) -> Result<Json<RecommendResponse>, AppError> {
    match query.strategy {
        Strategy::Random => recommend_random(&pool, user.user_id).await,
        Strategy::Ai => recommend_with_ai(&pool, user.user_id, ai::RecommendMode::FromList).await,
        Strategy::NewAi => recommend_with_ai(&pool, user.user_id, ai::RecommendMode::New).await,
    }
}

// ランダムに選ばれたおすすめ作品の表示
async fn recommend_random(pool: &PgPool, user_id: i32) -> Result<Json<RecommendResponse>, AppError> {
    match db::picked_random(pool, user_id).await? {
        Some(title) => Ok(Json(RecommendResponse { message: title, remaining: None })),
        None => Ok(Json(RecommendResponse {
            message: "おすすめできる作品がありません".to_string(),
            remaining: None,
        })),
    }
}

// AIが作品をレコメンドする機能
async fn recommend_with_ai(
    pool: &PgPool,
    user_id: i32,
    mode: ai::RecommendMode,
) -> Result<Json<RecommendResponse>, AppError> {
    let remaining = db::check_and_increment_ai_calls(pool, user_id, 10)
    .await?
    .ok_or(AppError::TooManyRequests(
        "本日のAI利用回数の上限に達しました".to_string()
    ))?;
    // 作品一覧取得
    let completed_work_list = db::get_list(pool, user_id, Some(&Status::Completed)).await?;
    let notstarted_work_list = db::get_list(pool, user_id, Some(&Status::NotStarted)).await?;

    // 0件の時の処理
    match mode {
        ai::RecommendMode::FromList => {
            if notstarted_work_list.is_empty() {
                return Ok(Json(RecommendResponse {
                    message: "未視聴作品がないためレコメンドできません".to_string(),
                    remaining: None,
                }));
            }
        }
        ai::RecommendMode::New => {
            if completed_work_list.is_empty() {
                return Ok(Json(RecommendResponse {
                    message: "視聴済み作品がないためレコメンドできません".to_string(),
                    remaining: None,
                }));
            }
        }
    }
    // 関数呼び出し
    let response_message = ai::recommend(completed_work_list, notstarted_work_list, mode).await?;

    // APIを呼ぶ
    Ok(Json(RecommendResponse {
        message: response_message,
        remaining: Some(remaining),
    }))
}

// 1リクエストで登録できるジャンル数の上限（過大入力によるDoS防止）
const MAX_GENRES: usize = 20;

// パスワード長の許容範囲（空パスワードや過大入力によるargon2負荷を防ぐ）
const PASSWORD_MIN_LEN: usize = 8;
const PASSWORD_MAX_LEN: usize = 128;

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
    user: AuthUser,
    Json(body): Json<WorkRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    if body.genres.len() > MAX_GENRES {
        return Err(AppError::BadRequest(format!(
            "ジャンルは最大{MAX_GENRES}個までです"
        )));
    }
    let work = Work {
        title: body.title,
        author: body.author,
        description: body.description,
        episodes: body.episodes,
        media_type: body.media_type,
        genres: body.genres,
    };
    // DB登録処理呼び出し
    db::insert_work(&pool, &work, user.user_id, &body.status).await?;
    Ok(Json(MessageResponse {
        message: "登録しました".to_string(),
    }))
}

// ステータス更新処理
#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: Status,
}

pub async fn update_work_status(
    State(pool): State<PgPool>,
    user: AuthUser,
    Path(work_id): Path<i32>,
    Json(body): Json<UpdateStatusRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let found = db::update_status(&pool, user.user_id, work_id, &body.status).await?;
    if !found {
        return Err(AppError::NotFound("作品が見つかりません".to_string()));
    }
    Ok(Json(MessageResponse {
        message: "ステータスを更新しました".to_string(),
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
    // パスワード長を検証（空パスワードや過大入力を弾く）
    let password_len = body.password.chars().count();
    if !(PASSWORD_MIN_LEN..=PASSWORD_MAX_LEN).contains(&password_len) {
        return Err(AppError::BadRequest(format!(
            "パスワードは{PASSWORD_MIN_LEN}〜{PASSWORD_MAX_LEN}文字で入力してください"
        )));
    }

    // 平文のパスワードをハッシュ化
    let password_hash = auth::hash_password(&body.password)?;

    let result = db::register(&pool, &body.name, &body.email, &password_hash).await;
    match result {
        Ok(_) => {}
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return Ok(Json(MessageResponse {
                        message: "ユーザーを登録しました".to_string(),
                    }));
                }
            }
            return Err(AppError::Database(e));
        }
    }
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

// メールアドレスとパスワードを照合し，成功したらJWTをCookieに発行
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<MessageResponse>), AppError> {
    let invalid = || AppError::Unauthorized("メールアドレスまたはパスワードが違います".to_string());

    let user = db::find_user_by_email(&state.pool, &body.email)
        .await?
        .ok_or_else(invalid)?;

    let password_hash = user.password_hash.ok_or_else(invalid)?;

    if !auth::verify_password(&body.password, &password_hash) {
        return Err(invalid());
    }

    // JWTを発行しhttpOnly Cookieに載せて返す
    let token = auth::create_token(user.id, user.token_version, &state.jwt_secret)?;
    let cookie = build_token_cookie(token, &state);

    Ok((
        jar.add(cookie),
        Json(MessageResponse {
            message: "ログインしました".to_string(),
        }),
    ))
}

// token Cookieを削除してログアウト（属性をlogin時と揃えないと削除が効かない）
pub async fn logout(
    State(state): State<AppState>,
    user: AuthUser,
    jar: CookieJar,
) -> Result<(CookieJar, Json<MessageResponse>), AppError> {
    db::increment_token_version(&state.pool, user.user_id).await?;

    let cookie = build_token_cookie(String::new(), &state);
    Ok((
        jar.remove(cookie),
        Json(MessageResponse {
            message: "ログアウトしました".to_string(),
        }),
    ))
}

// token Cookieを共通の属性で組み立てる
fn build_token_cookie(token: String, state: &AppState) -> Cookie<'static> {
    Cookie::build(("token", token))
        .http_only(true)
        .same_site(state.cookie_same_site)
        .secure(state.cookie_secure)
        .path("/")
        .build()
}

pub async fn ai_usage(
    State(pool): State<PgPool>,
    user: AuthUser,
) -> Result<Json<AiUsageResponse>, AppError> {
    const LIMIT: i32 = 10;
    let calls = sqlx::query_scalar!(
    "SELECT ai_calls_today FROM users WHERE id = $1",
    user.user_id
    )
    .fetch_optional(&pool)
    .await?
    .unwrap_or(0);

    Ok(Json(AiUsageResponse { remaining: (LIMIT - calls).max(0),
    limit: LIMIT,
    }))
}