use crate::db;
use crate::handlers;
use crate::state::AppState;
use anyhow::Result;
use axum::{Router, routing::get, routing::post};
use std::env;
use std::sync::Arc;

// 環境変数の読み込み → DB接続 → ルータ構築 → サーバ起動
pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = db::establish_connection().await;

    // JWT署名用の秘密鍵を読み込む
    let jwt_secret: Arc<str> = env::var("JWT_SECRET")
        .expect("JWT_SECRETを環境変数に設定してください")
        .into();

    let state = AppState { pool, jwt_secret };

    // ルータ及びハンドラ
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/list", get(handlers::list))
        .route("/recommendations", get(handlers::recommendations))
        .route("/works", post(handlers::create_work))
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .with_state(state);

    // TCPリスナの作成
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    let addr = listener.local_addr()?;
    println!("listening on http://{}", addr);

    // HTTPサーバの起動
    axum::serve(listener, app).await?;

    Ok(())
}
