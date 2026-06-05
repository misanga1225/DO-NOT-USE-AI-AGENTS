use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

// アプリ全体で使う共通のエラー型
#[derive(Debug)]
pub enum AppError {
    // DB由来のエラー
    Database(sqlx::Error),
    NotFound(String),
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
        };
        (status, Json(ErrorBody { message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}
