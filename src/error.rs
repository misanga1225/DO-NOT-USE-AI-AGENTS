use axum::Json;
use axum::extract::path::ErrorKind::Message;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::models::Status;

// アプリ全体で使う共通のエラー型
#[derive(Debug)]
pub enum AppError {
    // DB由来のエラー
    Database(sqlx::Error),
    NotFound(String),
    // AIなどの外部API連携で発生したエラー
    External(String),
}

// クライアントに返すJSONの形
#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

// AppError型をHTTPレスポンスに変換
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("database error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "サーバ内部でエラーが発生しました".to_string(),
                )
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::External(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        (status, Json(ErrorBody { message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}
