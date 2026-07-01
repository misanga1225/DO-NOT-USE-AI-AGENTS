use crate::db;
use crate::handlers;
use crate::state::AppState;
use anyhow::Result;
use axum::http::{HeaderValue, Method, header};
use axum::routing::patch;
use axum::{Router, routing::get, routing::post};
use axum_extra::extract::cookie::SameSite;
use std::env;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

// 環境変数の読み込み → DB接続 → マイグレーション → ルータ構築 → サーバ起動
pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = db::establish_connection().await;

    // 起動時にDBスキーマを最新へ追従させる
    sqlx::migrate!().run(&pool).await?;

    // JWT署名用の秘密鍵を読み込む
    let jwt_secret: Arc<str> = env::var("JWT_SECRET")
        .expect("JWT_SECRETを環境変数に設定してください")
        .into();

    let state = AppState {
        pool,
        jwt_secret,
        // 本番(HTTPS)ではCOOKIE_SECURE=true
        cookie_secure: env_bool("COOKIE_SECURE", false),
        // 別オリジン構成ではCOOKIE_SAMESITE=none
        cookie_same_site: parse_same_site(&env::var("COOKIE_SAMESITE").unwrap_or_default()),
    };

    // ルータ及びハンドラ
    let mut app = Router::new()
        .route("/", get(handlers::root))
        .route("/list", get(handlers::list))
        .route("/recommendations", get(handlers::recommendations))
        .route("/works", post(handlers::create_work))
        .route("/works/{id}", patch(handlers::update_work_status))
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .with_state(state);

    // 別オリジン構成のときだけCORSを有効化
    if let Some(cors) = cors_layer() {
        app = app.layer(cors);
    }

    // TCPリスナの作成（コンテナ外から到達できるよう 0.0.0.0 にバインド）
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    let addr = listener.local_addr()?;
    println!("listening on http://{}", addr);

    // HTTPサーバの起動
    axum::serve(listener, app).await?;

    Ok(())
}

// 真偽値の環境変数を読む
fn env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(v) => matches!(v.trim().to_ascii_lowercase().as_str(), "true" | "1"),
        Err(_) => default,
    }
}

// SameSite属性を文字列から決める
fn parse_same_site(value: &str) -> SameSite {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => SameSite::None,
        "strict" => SameSite::Strict,
        _ => SameSite::Lax,
    }
}

// CORS_ALLOWED_ORIGINS
fn cors_layer() -> Option<CorsLayer> {
    let raw = env::var("CORS_ALLOWED_ORIGINS").ok()?;
    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|o| !o.is_empty())
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    if origins.is_empty() {
        return None;
    }

    // Cookie認証のためallow_credentials(true)
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PATCH])
            .allow_headers([header::CONTENT_TYPE])
            .allow_credentials(true),
    )
}
