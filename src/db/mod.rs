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

// 作品を追加（genres は別テーブルに展開してリンクする）
pub async fn insert_work(pool: &PgPool, work: &Work) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let work_id: i32 = sqlx::query_scalar!(
        "INSERT INTO works (title, author, description, episodes, media_type, status)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        &work.title,
        &work.author,
        &work.description,
        work.episodes,
        work.media_type.as_str(),
        work.status.as_str(),
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

    tx.commit().await
}

// タイトル一覧をDBから取得
pub async fn get_list(pool: &PgPool, status: Option<&str>) -> Result<Vec<String>, sqlx::Error> {
    if let Some(status) = status {
        sqlx::query_scalar!("SELECT title FROM works WHERE status = $1", status)
            .fetch_all(pool)
            .await
    } else {
        sqlx::query_scalar!("SELECT title FROM works")
            .fetch_all(pool)
            .await
    }
}

pub async fn picked_random(pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar!(
        "SELECT title FROM works WHERE status = 'NotStarted' ORDER BY RANDOM() LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

// DB用のユニットテストだが，現状ヘルパー関数の置き場（後でどうにかする）
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Status;

    #[allow(dead_code)]
    fn filter_status<'a>(works: &'a [Work], status: &'a Status) -> Vec<&'a Work> {
        works.iter().filter(|w| w.status == *status).collect()
    }

    // 作品リストから特定の作品をお勧めする機能
    // 今はまだアルゴリズムがランダムだけど，将来的にはリストの中からAIに選ばせたい
    #[allow(dead_code)]
    pub fn pick_recommend(works: &[Work]) -> Option<&Work> {
        works
            .iter()
            .filter(|w| w.status == Status::NotStarted)
            .choose(&mut rand::rng())
    }
}
