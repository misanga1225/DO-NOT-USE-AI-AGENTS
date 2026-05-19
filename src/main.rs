// anyhowはまだ使ってないから警告出るけど消さないで
use anyhow::Result;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::types::chrono::{NaiveDate, Utc};

mod db;
mod handlers;
mod models;

use models::Work;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::establish_connection().await;

    // 動作確認用データのINSERT
    let work = Work {
        id: 0,
        title: String::from("とある魔術の禁書目録"),
        author: String::from("鎌池和馬"),
        description: String::from("学園都市の超能力者と魔術師たちの物語"),
        episodes: 24,
        media_type: String::from("Novel"),
        genre: String::from("ファンタジー"),
        status: String::from("InProgress"),
        added_at: NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
        created_at: Utc::now(),
    };
    db::insert_work(&pool, &work).await.unwrap();

    // ルータの作成
    // エンドポイントと，ハンドラと呼ばれるリクエストに対してどんなレスポンスを返すかの関数も後に実装
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/list", get(handlers::list))
        .with_state(pool);

    // TCPリスナの作成
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {:?}", listener.local_addr().unwrap());

    // HTTPサーバの起動
    axum::serve(listener, app).await.unwrap();
}
