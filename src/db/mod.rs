use crate::models::Work;
use sqlx::PgPool;
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

// 作品を追加
pub async fn insert_work(pool: &PgPool, work: &Work) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO animes (title, author, description, episodes, media_type, genre, status, added_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &work.title,
        &work.author,
        &work.description,
        work.episodes,
        work.media_type.as_str(),
        &work.genre,
        work.status.as_str(),
        &work.added_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

// タイトル一覧をDBから取得
pub async fn get_list(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let works = sqlx::query_scalar!("SELECT title FROM animes")
        .fetch_all(pool)
        .await?;

    Ok(works)
}
