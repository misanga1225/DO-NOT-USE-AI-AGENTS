use crate::error::AppError;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header, encode};
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
}

// user_idから署名済みJWTを発行
pub fn create_token(user_id: i32, secret: &str) -> Result<String, AppError> {
    let exp = (SystemTime::now() + Duration::from_secs(60 * 60 * 24))
        .duration_since(UNIX_EPOCH)
        .expect("システム時刻が不正です")
        .as_secs() as usize;

    let claims = Claims { sub: user_id, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::External(format!("トークンの発行に失敗しました: {e}")))
}
