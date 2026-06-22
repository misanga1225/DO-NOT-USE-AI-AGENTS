use crate::models::{Status, Work};
use sqlx::PgPool;
use sqlx::pool::maybe::MaybePoolConnection::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn establish_connection() -> PgPool {
    // .envからDATABASE_URLを読み込む
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLを環境変数に設定してください");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("プールの作成に失敗しました")
}

// 作品を追加（genres は別テーブルに展開してリンクする）
pub async fn insert_work(
    pool: &PgPool,
    work: &Work,
    user_id: i32,
    status: &Status,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let work_id: i32 = sqlx::query_scalar!(
        "INSERT INTO works (title, author, description, episodes, media_type)
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
        &work.title,
        &work.author,
        &work.description,
        work.episodes,
        work.media_type.as_str(),
    )
    .fetch_one(&mut *tx)
    .await?;

    for name in &work.genres {
        // 既存ジャンルなら id を取り、無ければ新規作成して id を返す
        let genre_id: i32 = sqlx::query_scalar!(
            "INSERT INTO genres (name) VALUES ($1)
             ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
             RETURNING id",
            name
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO work_genres (work_id, genre_id) VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
            work_id,
            genre_id
        )
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query!(
        "INSERT INTO user_works (user_id, work_id, status) VALUES ($1, $2, $3)
         ON CONFLICT (user_id, work_id) DO UPDATE SET status = EXCLUDED.status",
        user_id,
        work_id,
        status.as_str(),
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await
}

// タイトル一覧をDBから取得
pub async fn get_list(
    pool: &PgPool,
    user_id: i32,
    status: Option<&Status>,
) -> Result<Vec<String>, sqlx::Error> {
    if let Some(status) = status {
        sqlx::query_scalar!(
            "SELECT w.title
             FROM works w
             JOIN user_works uw ON uw.work_id = w.id
             WHERE uw.user_id = $1 AND uw.status = $2",
            user_id,
            status.as_str()
        )
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_scalar!(
            "SELECT w.title
             FROM works w
             JOIN user_works uw ON uw.work_id = w.id
             WHERE uw.user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
    }
}

pub async fn picked_random(pool: &PgPool, user_id: i32) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar!(
        "SELECT w.title
         FROM works w
         JOIN user_works uw ON uw.work_id = w.id
         WHERE uw.user_id = $1 AND uw.status = 'NotStarted'
         ORDER BY RANDOM() LIMIT 1",
        user_id
    )
    .fetch_optional(pool)
    .await
}

// 作品のステータス変更
pub async fn update_status(pool: &PgPool, user_id: i32, work_id: i32, status: &Status) -> Result<bool, sqlx::Error>{
    Ok(sqlx::query!(
        "UPDATE user_works SET status = $1 WHERE user_id = $2 AND work_id = $3",
        status.as_str(),
        user_id,
        work_id,
    )
    .execute(pool)
    .await?
    .rows_affected()
        > 0)
}

// ログイン照合に必要な最小限のユーザ情報
pub struct UserCredentials {
    pub id: i32,
    pub password_hash: Option<String>,
}

// 新規ユーザーを登録
pub async fn register(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
        name,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
}

// メールアドレスから照合用のユーザ情報を引く
pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserCredentials>, sqlx::Error> {
    sqlx::query_as!(
        UserCredentials,
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await
}

// 認証導入までの動作確認用
pub async fn ensure_demo_user(pool: &PgPool) -> Result<i32, sqlx::Error> {
    if let Some(id) = sqlx::query_scalar!("SELECT id FROM users WHERE name = 'demo'")
        .fetch_optional(pool)
        .await?
    {
        return Ok(id);
    }
    sqlx::query_scalar!("INSERT INTO users (name) VALUES ('demo') RETURNING id")
        .fetch_one(pool)
        .await
}
