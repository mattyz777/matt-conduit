//! 通用应用错误
//!
//! 统一错误类型，各层共享，通过 `#[from]` 自动转换，避免频繁手动转换

use thiserror::Error;
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::Serialize;

#[derive(Error, Debug)]
pub enum AppError {
    // 基础设施层（DAO）
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("redis error: {0}")]
    Redis(String),

    // 业务层（Service）
    #[error("user not found")]
    UserNotFound,

    #[error("username exists: {0}")]
    UsernameExists(String),

    #[error("invalid password")]
    InvalidPassword,

    // 认证/授权
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden")]
    Forbidden,

    // 请求层（Handler）
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),
}

/// 标准错误响应格式
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    data: serde_json::Value,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code): (StatusCode, u16) = match &self {
            // ========== 4xx 错误 ==========
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, 400),
            AppError::Unauthorized(_) | AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, 401),
            AppError::Forbidden => (StatusCode::FORBIDDEN, 403),
            AppError::NotFound(_) | AppError::UserNotFound => (StatusCode::NOT_FOUND, 404),
            AppError::UsernameExists(_) | AppError::Conflict(_) => (StatusCode::CONFLICT, 409),

            // ========== 5xx 错误 ==========
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
        };

        let body = ErrorResponse {
            code,
            message: self.to_string(),
            data: serde_json::Value::Null,
        };

        (status, Json(body)).into_response()
    }
}
