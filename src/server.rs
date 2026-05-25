use crate::db;
use crate::handlers;
use crate::models::{MediaType, Status, Work};
use anyhow::Result;
use axum::{Router, routing::get};
use sqlx::types::chrono::{NaiveDate, Utc};

// 環境変数の読み込み → DB接続 → ルータ構築 → サーバ起動
pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::establish_connection().await;

    // 動作確認用データのINSERT
    let work = Work {
        id: 0,
        title: String::from("とある魔術の禁書目録"),
        author: String::from("鎌池和馬"),
        description: String::from("学園都市の超能力者と魔術師たちの物語"),
        episodes: 24,
        media_type: MediaType::Novel,
        genre: String::from("ファンタジー"),
        status: Status::InProgress,
        added_at: NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
        created_at: Utc::now(),
    };
    db::insert_work(&pool, &work).await?;

    // ルータ及びハンドラ
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/list", get(handlers::list))
        .route("/recommend", get(handlers::random))
        .with_state(pool);

    // TCPリスナの作成
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    let addr = listener.local_addr()?;
    println!("listening on http://{}", addr);

    // HTTPサーバの起動
    axum::serve(listener, app).await?;

    Ok(())
}
