use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

// アプリ全体で共有する状態
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<str>,
}

// 既存ハンドラがState<PgPool>のまま受け取れるようにする
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
