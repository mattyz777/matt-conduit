//! 通用错误类型定义
//!
//! 提供应用程序中通用的错误类型定义

use thiserror::Error;
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use crate::dto::common_dto::ApiResponse;

/// 通用应用错误
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServer(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

/// 错误码
pub type ErrorCode = u16;

impl AppError {
    /// 获取错误码
    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::InternalServer(_) => 5000,
            AppError::BadRequest(_) => 4000,
            AppError::Unauthorized(_) => 4010,
            AppError::NotFound(_) => 4040,
            AppError::AlreadyExists(_) => 4090,
            AppError::Conflict(_) => 4090,
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> String {
        match self {
            AppError::InternalServer(msg) => msg.clone(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::AlreadyExists(msg) => msg.clone(),
            AppError::Conflict(msg) => msg.clone(),
        }
    }
}

// 实现 From<sea_orm::DbErr> for AppError
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        tracing::error!("Database error: {:?}", err);
        AppError::InternalServer(err.to_string())
    }
}

// 实现 IntoResponse for AppError
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Conflict(_) => StatusCode::CONFLICT,
        };

        let body = ApiResponse::error(self.code(), &self.message());

        (status, Json(body)).into_response()
    }
}
