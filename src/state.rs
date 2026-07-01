use axum::extract::FromRef;
use axum_extra::extract::cookie::SameSite;
use sqlx::PgPool;
use std::sync::Arc;

// アプリ全体で共有する状態
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<str>,
    // 認証Cookieの属性
    pub cookie_secure: bool,
    pub cookie_same_site: SameSite,
}

// 既存ハンドラがState<PgPool>のまま受け取れるようにする
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
