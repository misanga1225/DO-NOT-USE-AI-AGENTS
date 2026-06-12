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
    // クライアントの入力が不正なときのエラー
    BadRequest(String),
    // 認証に失敗したときのエラー
    Unauthorized(String),
    // 既に存在するメールアドレスなど競合したときのエラー
    Conflict(String),
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
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn notfound() {
        let res = AppError::NotFound("なし".into()).into_response();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn external() {
        let res = AppError::External("api error".into()).into_response();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
