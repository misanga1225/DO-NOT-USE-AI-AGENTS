use crate::error::AppError;
use crate::state::AppState;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// 平文パスワードをargon2でハッシュ化
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::External(format!("パスワードのハッシュ化に失敗しました: {e}")))
}

// 平文パスワードと保存済みハッシュを照合
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    match PasswordHash::new(password_hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        // パースできない場合は不一致扱い
        Err(_) => false,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    // 発行時点のtoken_version
    pub ver: i32,
}

// user_idから署名済みJWTを発行
pub fn create_token(user_id: i32, token_version: i32, secret: &str) -> Result<String, AppError> {
    let exp = (SystemTime::now() + Duration::from_secs(60 * 60 * 24))
        .duration_since(UNIX_EPOCH)
        .expect("システム時刻が不正です")
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id,
        exp,
        ver: token_version,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::External(format!("トークンの発行に失敗しました: {e}")))
}

pub struct AuthUser {
    pub user_id: i32,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let unauthorized = || AppError::Unauthorized("認証が必要です".to_string());

        // リクエストからCookieを取り出す
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| unauthorized())?;

        // token Cookieを取得
        let token = jar
            .get("token")
            .ok_or_else(unauthorized)?
            .value()
            .to_string();

        // 署名と有効期限を検証し，中身を取り出す
        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| unauthorized())?
        .claims;

        // トークンのバージョンが現在のユーザのものと一致するか確認（失効チェック）
        let current_version = crate::db::get_token_version(&state.pool, claims.sub)
            .await
            .map_err(|_| unauthorized())?
            .ok_or_else(unauthorized)?;

        if claims.ver != current_version {
            return Err(unauthorized());
        }

        // 検証済みのuser_idを返す
        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
